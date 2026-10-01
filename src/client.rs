use futures_util::{StreamExt, TryStreamExt, future};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_stream::wrappers::LinesStream;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

#[tokio::main]
async fn main() {
    let (ws_stream, _response) = connect_async("ws://127.0.0.1:9001")
        .await
        .expect("connect failed");

    let (write, read) = ws_stream.split();

    let stdin_to_ws = LinesStream::new(BufReader::new(tokio::io::stdin()).lines())
        .map_ok(|line| Message::Text(line.into()))
        .err_into()
        .forward(write);

    let ws_to_stdout = read
        .try_filter_map(|msg| future::ready(Ok(msg.into_text().ok())))
        .try_for_each(|text| async move {
            println!("{text}");
            Ok(())
        });

    if let Err(e) = future::try_join(stdin_to_ws, ws_to_stdout).await {
        eprintln!("error: {e}");
    }
}
