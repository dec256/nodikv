use std::collections::HashMap;
use std::sync::Arc;
use tokio::{
    net::{TcpListener, TcpStream},
    io::{AsyncReadExt, AsyncWriteExt},
    sync::Mutex,
};
use anyhow::Result;
use chrono::{Utc, Timelike};

const ADDRESS: &str = "127.0.0.1:8081";

enum States {
    Follower,
    Candidate,
    Leader,
}

type Db = Arc<Mutex<HashMap<String, String>>>;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = tokio::spawn(raft()).await?;
    let listener = TcpListener::bind("0.0.0.0:80").await?;
    let db: Db = Arc::new(Mutex::new(HashMap::new()));
    
    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("\nconnection from {}", addr);

        let db = Arc::clone(&db);
        tokio::spawn(async move {
            let mut buf = Vec::new();
            if let Err(e) = socket.read_to_end(&mut buf).await {
                eprintln!("failed to read from socket: {}", e);
                return;
            }

            let data = String::from_utf8_lossy(&buf);
            if let Err(e) = parse(data.to_string(), socket, db).await {
                eprintln!("error processing request: {}", e);
            }
        });
    }
}

async fn parse(data: String, mut stream: TcpStream, db: Db) -> Result<()> {
    println!("{}", data);
    if data.contains("PUSH") {
        let lines: Vec<&str> = data.split_whitespace().collect();
        if lines.len() != 3 {
            return Err(anyhow::anyhow!("does not contain all 3 PUSH key val"));
        }
        push(&db, lines[1], lines[2]).await;
        println!("PUSHED");
    } else if data.contains("PULL") {
        let lines: Vec<&str> = data.split_whitespace().collect();
        if lines.len() != 2 {
            return Err(anyhow::anyhow!("does not contain all PULL key"));
        }
        let value = pull(&db, lines[1]).await;
        stream.write_all(value.as_bytes()).await?;
        println!("{}", value);
    }

    Ok(())
}

async fn push(db: &Db, key: &str, val: &str) {
    let mut db = db.lock().await;
    db.insert(key.to_string(), val.to_string());
}

async fn pull(db: &Db, key: &str) -> String {
    println!("this is the data for {}", key);
    let db = db.lock().await;
    db.get(key).cloned().unwrap_or_default()
}       

async fn raft() -> Result<()> {
    let mut current = States::Follower;
    let listener = TcpListener::bind(ADDRESS).await?;
    loop {
        let now = Utc::now();
        let secs = now.num_seconds_from_midnight();
        //let secs = (seconds_since_midnight + 5) % 86400;
        println!("{}", secs);
    }
    Ok(())
}
