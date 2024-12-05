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
    let mut path = vec![];

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
        write_to_server(&mut stream, ccmd.split_bytes(), path.split_bytes()).await?;

        // Get response from server
        let mut buf = vec![];
        let _bytes = stream.read_buf(&mut buf).await?;

        // don't use println since output may contain invalid bytes
        std::io::stdout().write_all(&buf)?;
        println!("");
        std::io::stdout().flush()?;

        return Ok(())
    }

    // Start in "play-mode"
    println!("NOSQL CLIENT");

    loop {
        // Read input
        print!("CrashDB");
        for j in &path {print!("/{j}")}
        print!("> ");
        let _ = std::io::stdout().flush();

        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input);
        let mut instream = input.split_whitespace();

        // Local commands
        match instream.next() {
            Some("exit") => {
                println!();
                let _ = stream.shutdown().await;
                break;
            },
            Some("open") => {
                path.push(instream.next().unwrap().to_owned());
                continue;
            }
            Some("close") => {
                path.pop();
                continue;
            }
            _ => {}
        }


        // Send command to server
        write_to_server(&mut stream, input.split_bytes(), path.split_bytes()).await?;

        // Get response from server
        let mut buf = vec![];
        let _bytes = stream.read_buf(&mut buf).await?;

        println!("{}", String::from_utf8(buf)?);
    }

    Ok(())
}

async fn write_to_server(stream: &mut TcpStream, cmdbytes: Vec<u8>, pathbytes: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    // write everything in one write
    stream.write_all(&vec![
        &u32_to_bytes(cmdbytes.len() as u32) as &[u8],
        &cmdbytes[..],
        &pathbytes[..]
    ].concat()).await?;
    stream.flush().await?;
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