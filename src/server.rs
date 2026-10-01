use futures_util::{StreamExt, TryStreamExt, future};
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tokio_tungstenite::accept_async;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:9001").await.unwrap();
    println!("Listening on ws://127.0.0.1:9001");

    TcpListenerStream::new(listener)
        .for_each_concurrent(None, |conn| async move {
            match conn {
                Ok(stream) => handle_connection(stream).await,
                Err(e) => eprintln!("accept error: {e}"),
            }
        })
        .await;
}

async fn handle_connection(stream: tokio::net::TcpStream) {
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => return eprintln!("handshake failed: {e}"),
    };

    let (write, read) = ws_stream.split();

    let echo = read
        .try_filter(|msg| future::ready(msg.is_text() || msg.is_binary()))
        .inspect_ok(|msg| println!("Received: {msg}"))
        .forward(write);

    if let Err(e) = echo.await {
        eprintln!("echo stream error: {e}");
    }
}
