use std::net::SocketAddr;

use axum::{
    extract::{ws::WebSocket, ConnectInfo, WebSocketUpgrade},
    response::IntoResponse,
    routing::any,
    Router,
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:5678").await?;

    let app = Router::new().route("/", any(ws_handler));

    axum::serve(listener, app).await.unwrap();

    println!("Hello, world!");
    Ok(())
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}

async fn handle_socket(mut socket: WebSocket, who: SocketAddr) {
    if let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            println!("message: {:?}", msg);
            return;
        } else {
            println!("client {:?} abruptly disconnected", who);
            return;
        }
    }
}
