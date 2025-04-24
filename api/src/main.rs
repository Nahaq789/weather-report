use axum::{
    extract::{ws::WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::any,
    Router,
};
use tokio::net::TcpListener;

pub mod dynamodb;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:5678").await?;

    let app = Router::new().route("/ws", any(ws_handler));

    axum::serve(listener, app).await.unwrap();

    println!("Hello, world!");
    Ok(())
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket))
}

async fn handle_socket(mut socket: WebSocket) {
    if let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            println!("message: {:?}", msg);

            let res = format!("こんにちは！ {:?}さん", msg);
            if socket.send(res.into()).await.is_err() {
                return;
            }
            return;
        } else {
            return;
        }
    }
}
