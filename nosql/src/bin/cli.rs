use tokio::{io::AsyncWriteExt, io::AsyncReadExt, net::TcpStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    println!("NOSQL client");

    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

    loop {
        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input);

        if input == "exit\n" {
            let _ = stream.shutdown().await;
            break;
        }

        stream.write_all(input.as_bytes()).await?;
        stream.flush().await?;

        let mut buf = vec![];

        let _bytes = stream.read_buf(&mut buf).await?;

        println!("{:?}", String::from_utf8(buf)?);
    }

    Ok(())
}