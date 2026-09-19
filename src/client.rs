use tokio::{
    net::{TcpStream},
    io::{AsyncWriteExt, AsyncReadExt},
};

use std::{
    io::{Write},
};

use anyhow::Result;

const SEND_ADDRESS: &str = "127.0.0.1:8080";

fn input(data: &str)-> Result<String> {
    let mut to_send = String::new();
    print!("{}", &data);
    std::io::stdout().flush()?;
    std::io::stdin().read_line(&mut to_send)?;

    Ok(to_send)
}

#[tokio::main]
async fn main() -> Result<()> {
    loop{
        let mut socket = TcpStream::connect(SEND_ADDRESS).await?;

        let to_send = String::from(input("data: ")?);

        if to_send.trim().to_lowercase() == "end" {
            break;
        }
        if !to_send.trim().contains("PUSH") && !to_send.trim().contains("PULL") {
            println!("its either PULL or PUSH");
            continue;
        }
    
        socket.write_all(to_send.as_bytes()).await?;
        socket.shutdown().await?;  

        if to_send.trim().contains("PULL") {
            let mut data: Vec<u8> = Vec::new();
            let mut buf = [0u8; 4096];
            loop {
                let n = socket.read(&mut buf).await?;
                if n == 0 { break; }
                data.extend_from_slice(&buf[..n]);
            }
            println!("{}", String::from_utf8_lossy(&data));
        }
    }
    Ok(())
}
