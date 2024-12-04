use std::io::Write;
use nosql::utils::u32_to_bytes;
use tokio::{io::AsyncWriteExt, io::AsyncReadExt, net::TcpStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    // Set defaults
    let mut ip = "127.0.0.1";
    let mut port = "8080";

    // Take command line arguments
    let args: Vec<String> = std::env::args().into_iter().skip(1).collect();
    let mut skips = 0;
    let mut ccmd = vec![];

    let empty = String::new();

    for (arg1, arg2) in args.clone().into_iter().zip(args.iter().skip(1).chain([&empty])) {        
        if skips > 0 { skips -= 1; continue; }
        match arg1.as_str() {
            "ip" => {ip = &arg2;skips += 1;}
            "port" => {port = &arg2;skips += 1;}
            a => {ccmd.push(a.to_owned());}
        }
    }

    let mut stream = TcpStream::connect(format!("{}:{}", ip, port)).await?;

    // check if command passed through arguments
    if ccmd.len() > 0 {
        // Send command to server
        stream.write_all(&ccmd.split_bytes()).await?;
        stream.flush().await?;

        // Get response from server
        let mut buf = vec![];
        let _bytes = stream.read_buf(&mut buf).await?;

        // don't use println since output may contain invalid bytes
        // TODO: make pretty if not passed through command line args
        std::io::stdout().write_all(&buf)?;
        println!("");
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
            println!();
            let _ = stream.shutdown().await;
            break;
        }

        // Send command to server
        stream.write_all(&input.split_bytes()).await?;
        stream.flush().await?;

        // Get response from server
        let mut buf = vec![];
        let _bytes = stream.read_buf(&mut buf).await?;

        println!("{}", String::from_utf8(buf)?);
    }

    Ok(())
}

trait Splittable {
    fn split_bytes(&self) -> Vec<u8>;
}
impl Splittable for String {
    fn split_bytes(&self) -> Vec<u8>{
        let mut bvec = vec![];

        for nextstr in self.split_whitespace() {
            let nbytes = nextstr.as_bytes();
            bvec.extend(u32_to_bytes(nbytes.len() as u32));
            bvec.extend(nbytes);
        }
        bvec
    }
}

impl Splittable for Vec<String> {
    fn split_bytes(&self) -> Vec<u8> {
        let mut bvec = vec![];
        
        for word in self {
            let nbytes = word.as_bytes();
            bvec.extend(u32_to_bytes(nbytes.len() as u32));
            bvec.extend(nbytes);
        }

        bvec
    }
}