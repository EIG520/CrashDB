use nosql::commands::commands::DbHandler;
//use nosql::commands;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::Arc;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("NOSQL SERVER");

    // Open connection to localhost
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
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

                        let s = &grecv.recv().await.unwrap();

                        // gracefully exit (mostly ignore the error)
                        if let Err(_) = socket.write_all(s).await {
                            break;
                        }
                    }
                });
            }
        }
    });

    // Main database
    let mut dbhandler = DbHandler::default();

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

                    sender.send(b"no.".to_vec()).await?;
                }
            }
        }
    }
}

