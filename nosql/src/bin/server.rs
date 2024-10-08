use nosql::commands::commands::DbHandler;
//use nosql::commands;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
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
    let (cmdsender, mut cmdlistener) = mpsc::channel::<(String, mpsc::Sender<Vec<u8>>)>(1000);

    tokio::spawn(async move { 

        // thing to send commands to main loop
        let fcmdsender = Arc::new(cmdsender);

        // Look for new clients trying to connect
        loop {

            // Local thing to send commands to main loop
            let rcmdsender = fcmdsender.clone();

            // New client trying to connect
            if let Ok((mut socket, _addr)) = listener.accept().await {

                tokio::spawn( async move {

                    // Send commands from client to main loop
                    loop {
                        let mut buf = vec![];

                        let _ = socket.read_buf(&mut buf).await;

                        let (gsend, mut grecv) = mpsc::channel::<Vec<u8>>(1);

                        let _ = rcmdsender.send((String::from_utf8(buf).unwrap(), gsend)).await;

                        let mut resp = &vec![];

                        let tresp = &grecv.recv().await;

                        if let Some(s) = tresp {
                            resp = s;
                        } 

                        // gracefully exit (ignore the error)
                        if let Err(_) = socket.write_all(&resp).await {
                            break;
                        }
                    }
                });
            }
        }
    });


    // Main loop for handling commands
    loop {
        // Get next command
        if let Some((cmd, sender)) = cmdlistener.recv().await {
            println!("executing command: {:?}", cmd);

            match dbhandler.handle_command(&mut cmd.split_whitespace()) {
                Ok(res) => {
                    let vres = res.to_vec();

                    println!("result: {:?}", String::from_utf8(vres.clone()));

                    sender.send(vres).await?;
                },
                Err(e) => {
                    println!("result: {:?}", e);

                    sender.send(b"command could not be executed".to_vec()).await?;
                }
            }
        }
    }
}

