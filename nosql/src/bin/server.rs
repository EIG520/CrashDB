use nosql::commands::commands::DbHandler;
use nosql::data_types::data_types::SavableType;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("NOSQL SERVER");

    // Set defaults
    let mut ip = "127.0.0.1";
    let mut port = "8080";
    let mut maybe_path: Option<&str> = None;

    // Take command line arguments
    let args: Vec<String> = std::env::args().into_iter().collect();

    for (arg1, arg2) in args.clone().iter().zip(args.iter().skip(1)) {
        match arg1.as_str() {
            "ip" => {ip = &arg2;}
            "port" => {port = &arg2;}
            "file" => {maybe_path = Some(&arg2)}
            _ => {}
        }
    }

    // Main database
    let mut dbhandler = DbHandler::default();

    if let Some(path) = maybe_path {
        dbhandler.dump_path = path.to_owned();

        dbhandler.handle_full_load()?;
    }

    // Open connection to localhost
    let listener = TcpListener::bind(format!("{ip}:{port}")).await?;
    // sender to send senders so the listening thread can send responses back
    // surely there is no better solution
    let (cmdsender, mut cmdlistener) = mpsc::channel::<(String, Vec<String>, Vec<String>, mpsc::Sender<Vec<u8>>)>(1000);

    tokio::spawn(async move { 

        // thing to send commands to main loop
        let fcmdsender = Arc::new(cmdsender);

        // Look for new clients trying to connect
        loop {
            listen_for_client(fcmdsender.clone(), &listener).await;
        }
    });


    // Main loop for handling commands
    loop {
        // Get next command
        if let Some((first, cmdf, dir, sender)) = cmdlistener.recv().await {
            println!("executing command: {:?}", cmdf);

            let mut table: Rc<RefCell<SavableType>> = dbhandler.data.clone();
            let cmd = cmdf;

            for name in dir.clone() {
                match &mut *(table.clone().borrow_mut()) {
                    SavableType::Table(t) => {
                        table = t.load(name.clone())?
                    }
                    _ => {}
                };
            }

            if let SavableType::Table(t) = &mut *table.borrow_mut() {
                match t.handle_command(&first, cmd.iter().map(|i| i.as_str())) {
                    Ok(res) => {
                        println!("result: {:?}", String::from_utf8(res.clone()));

                        sender.send(res).await?;
                    },
                    Err(e) => {
                        println!("result: {:?}", e);

                        sender.send(b"command could not be executed".to_vec()).await?;
                    }
                };
            };
        }
    }
}

async fn listen_for_client(fcmdsender: Arc<mpsc::Sender<(String, Vec<String>, Vec<String>, mpsc::Sender<Vec<u8>>)>>, listener: &TcpListener) {
    // New client trying to connect
    if let Ok((mut socket, _addr)) = listener.accept().await {
        // Local thing to send commands to main loop
        let rcmdsender = fcmdsender.clone();
        
        tokio::spawn( async move {
            let mut dir: Vec<String> = vec![];

            // Send commands from client to main loop
            loop {
                let mut buf = vec![];

                let _ = socket.read_buf(&mut buf).await;
                let ucmd = String::from_utf8(buf).unwrap();
                let mut cmd = ucmd.split_whitespace().map(|x| x.to_owned());

                let (gsend, mut grecv) = mpsc::channel::<Vec<u8>>(1);
                
                // intercept open command

                // I'm not proud of this
                let first = match cmd.next() {
                    Some(t) => {
                        match t.as_str() {
                            "open" => {
                                if let Some(name) = cmd.next() {
                                    dir.push(name.to_owned());
                                }
                                if let Err(_) = socket.write_all(b"opened").await {
                                    break;
                                }
                                continue;
                            },
                            "close" => {
                                match dir.pop() {
                                    Some(t) => {
                                        if let Err(_) = socket.write_all(format!("closed file {:?}", t).as_bytes()).await {
                                            break;
                                        }
                                        continue;
                                    },
                                    _ => {
                                        if let Err(_) = socket.write_all(b"file couldn't be closed").await {
                                            break;
                                        }
                                        continue;
                                    }
                                };
                            },
                            _ => {t}
                        }

                    },
                    _ => {
                        if let Err(_) = socket.write_all(b"invalid command").await {
                            break;
                        }
                        continue;
                    },
                };

                let _ = rcmdsender.send((first.to_owned(), cmd.collect::<Vec<String>>().clone(), dir.clone(), gsend)).await;

                let mut resp = &vec![];

                let tresp: &Option<Vec<u8>> = &grecv.recv().await;

                if let Some(s) = tresp {
                    resp = s;
                }

                // 'gracefully' exit (ignore the error)
                if let Err(_) = socket.write_all(&resp).await {
                    break;
                }
            }
        });
    }
}