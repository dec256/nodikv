use tokio::{
    net::{TcpListener, TcpStream},
    io::{AsyncReadExt, AsyncWriteExt},
};

use std::{
    io::{Write},
};

use anyhow::Result;

const ADDRESS: &str = "127.0.0.1:8080";
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
    tokio::spawn(send());
    let listener = TcpListener::bind(ADDRESS).await?;
    
    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("\nconnection from {}", addr);

        let mut buf = Vec::new();
        socket.read_to_end(&mut buf).await?;

        let data = String::from_utf8_lossy(&buf);
        
        if data.contains("PULL!") {
            println!("It was pull"); //will be used to collect data
        } else if data.contains("ADD!") {
           println!("it was an add operation"); //will be used to add a node to the raft
        } else if data.contains("PUSH!") {
            println!("it was a push operation"); //wil be used to add data
        } else if data.contains("CAST!") {
            println!("it was a cast operation"); //will be used to ask for votes
        } else if data.contains("ELECTED!") {
            println!("it was an elected operation"); //leader was elected
        } else if data.contains("VOTE!") {
            println!("it was a vote operation"); //to vote for a casting node
        }
    }
}

async fn send() -> Result<()> {
    loop{
        let mut socket = TcpStream::connect(SEND_ADDRESS).await?;

        let to_send = String::from(input("data: ")?);

        if to_send.trim().to_lowercase() == "end" {
            break;
        }

        socket.write_all(to_send.as_bytes()).await?;

        let data: Vec<u8> = vec![1, 2, 3, 4, 5];
        socket.write_all(&data).await?;

        socket.flush().await?;
    }
    Ok(())
}
