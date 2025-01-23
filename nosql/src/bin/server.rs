use nosql::commands::commands::DbHandler;
use nosql::data_types::data_types::{Loadable, SavableType};
use nosql::utils::{bytes_to_usize, u32_to_bytes};
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

            // Run command if it is directory-independent
            if let Ok(resp) = dbhandler.handle_command(&first, cmdf.iter().map(|i| i.as_str())) {
                if resp != b"unknown command" { 
                    sender.send(resp).await?;
                    continue;
                }
            }

            // Open path
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
            // Send commands from client to main loop
            loop {
                let mut buf= vec![0u8; 4];
                let _ = socket.read_exact(&mut buf).await;
                let bcount = bytes_to_usize(buf);

                let mut cbuf = vec![0u8; bcount];
                let _ = socket.read_exact(&mut cbuf).await;
                let mut cmd = bytes_to_strvec(cbuf).into_iter().map(|x| x.to_owned());

                let mut buf = vec![0u8; 4];
                let _ = socket.read_exact(&mut buf).await;
                println!("{}", bytes_to_usize(buf.clone()));
                let mut dirbuf = vec![0u8; bytes_to_usize(buf)];
                let _ = socket.read_exact(&mut dirbuf).await;
                let dir= bytes_to_strvec(dirbuf);

                let (gsend, mut grecv) = mpsc::channel::<Vec<u8>>(1);
                
                let first = match cmd.next() {
                    Some(t) => { t },
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

                println!("{:?}", String::from_utf8(resp.to_vec()));

                // 'gracefully' exit (ignore the error)
                if let Err(_) = socket.write_all(&u32_to_bytes(resp.len() as u32)).await {
                    break;
                }

                if let Err(_) = socket.write_all(&resp).await {
                    break;
                }
                let _ = socket.flush().await;
            }
        });
    }
}

pub fn bytes_to_strvec(bytes: Vec<u8>) -> Vec<String> {
    let mut svec = vec![];

    let mut idx = 0;
    while idx < bytes.len() {
        let size = bytes_to_usize(vec![bytes[idx], bytes[idx + 1], bytes[idx + 2], bytes[idx + 3]]);
        idx += 4;

        let str_bytes = bytes[idx..(idx+size)].to_vec();

        svec.push(String::from_bin(&str_bytes));
        idx += size;
    }

    svec
}