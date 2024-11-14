use std::io::Write;

use tokio::{io::AsyncWriteExt, io::AsyncReadExt, net::TcpStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    // Set defaults
    let mut ip = "127.0.0.1";
    let mut port = "8080";

    let mut cmd: Option<&str> = None;

    // Take command line arguments
    let args: Vec<String> = std::env::args().into_iter().collect();

    for (arg1, arg2) in args.clone().iter().zip(args.iter().skip(1)) {
        match arg1.as_str() {
            "ip" => {ip = &arg2;}
            "port" => {port = &arg2;}
            "run" => {cmd = Some(&arg2);}
            _ => {}
        }
    }

    let mut stream = TcpStream::connect(format!("{}:{}", ip, port)).await?;

    if let Some(rcmd) = cmd {
        // Send command to server
        stream.write_all(rcmd.as_bytes()).await?;
        stream.flush().await?;

        // Get response from server
        let mut buf = vec![];
        let _bytes = stream.read_buf(&mut buf).await?;

        // don't use println since output may contain invalid bytes
        // TODO: make pretty if not passed through command line args
        std::io::stdout().write_all(&buf)?;
        std::io::stdout().flush()?;

        return Ok(())
    }

    println!("NOSQL CLIENT");

    loop {
        // Read input
        print!("CrashDB> ");
        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input);

        if input == "exit\n" {
            let _ = stream.shutdown().await;
            break;
        }

        // Send command to server
        stream.write_all(input.as_bytes()).await?;
        stream.flush().await?;

        // Get response from server
        let mut buf = vec![];
        let _bytes = stream.read_buf(&mut buf).await?;

        println!("{}", String::from_utf8(buf)?);
    }

    Ok(())
}