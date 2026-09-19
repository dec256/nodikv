use tokio::{
    fs::File,
    net::{TcpListener, TcpStream},
    io::{AsyncReadExt, AsyncWriteExt},
};
use anyhow::Result;

const ADDRESS: &str = "127.0.0.1:8080";
const FILE: &str = "nodikv.db";

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind(ADDRESS).await?;
    
    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("\nconnection from {}", addr);

        let mut buf = Vec::new();
        socket.read_to_end(&mut buf).await?;

        let data = String::from_utf8_lossy(&buf);
        let _ = parse(data.to_string(), socket).await?;
    }
}

async fn parse(data: String, mut stream: TcpStream) -> Result<()> {
    println!("{}", data);
    if data.contains("PUSH") {
        let lines: Vec<&str> = data.split_whitespace().collect();
        if lines.len() != 3 {
            return Err(anyhow::anyhow!("does not contain all 3 PUSH key val"));
        }
        let _ = push(lines[1], lines[2]).await?;
        println!("PUSHED");
    } else if data.contains("PULL") {
        let lines: Vec<&str> = data.split_whitespace().collect();
        if lines.len() != 2 {
            return Err(anyhow::anyhow!("does not contain all PULL key"));
        }
        let value = pull(lines[1]).await?;
        stream.write_all(value.as_bytes()).await?;
        println!("{}", value);
    }

    Ok(())
}

async fn push(key: &str, val: &str) -> Result<()> {
    let mut contents = String::new();
    {
        let mut file = File::open(FILE).await?;
        file.read_to_string(&mut contents).await?;
    }

    let mut new_contents = String::new();
    for line in contents.lines() {
        let components: Vec<&str> = line.splitn(2, ':').collect();
        if components[0] != key {
            new_contents.push_str(line);
            new_contents.push('\n');
        }
    }
    new_contents.push_str(&format!("{}\n", format!("{}:{}", key, val)));

    let mut file = File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(FILE)
        .await?;
    file.write_all(new_contents.as_bytes()).await?;
    Ok(())
}

async fn pull(key: &str) -> Result<String> {
    println!("this is the data for {}", key);
    let mut file = File::open(FILE).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;

    let parts: Vec<&str> = contents.split('\n').collect();

    for i in parts {
        let components: Vec<&str> = i.split(':').collect();
        if components[0] == key {
            return Ok(components[1].to_string());
        }
    }
    Ok(String::new())
}
