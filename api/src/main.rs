use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::any,
    Router,
};
use tokio::net::TcpListener;

pub mod dynamodb;
pub mod repository;

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
    println!("connection started");
    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            match msg {
                Message::Text(text) => {
                    println!("{}", text);

                    if socket.send(Message::from(text)).await.is_err() {
                        break;
                    }
                }
                Message::Close(_) => {
                    break;
                }
                _ => {
                    break;
                }
            }
        } else {
            break;
        }
    }
}
