use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::any,
    Router,
};
use dynamodb::build_client;
use repository::SensorRepositoryImpl;
use sensor::{
    repository::SensorRepository,
    sensor::measurements::{humidity, temperature},
};
use tokio::net::TcpListener;

pub mod dynamodb;
pub mod repository;
pub mod sensor_dto;

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
    let client = build_client().await.unwrap();
    let repository = SensorRepositoryImpl::new(client);

    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            match msg {
                Message::Text(text) => {
                    println!("{}", text);
                    let result = repository.get_sensor_data(&text).await.unwrap();
                    let mut tasks = Vec::new();

                    let mut temperature_vec: Vec<f64> = Vec::new();
                    let mut humidity_vec: Vec<f64> = Vec::new();

                    let task = tokio::spawn(async move {
                        let _t = result.iter().map(|v| {
                            temperature_vec.push(v.measurements().temperature().value());
                            humidity_vec.push(v.measurements().humidity().value());
                        });

                        println!("{:?}", temperature_vec);
                    });

                    tasks.push(task);

                    for task in tasks {
                        let _ = task.await.unwrap();
                    }

                    let message = Message::Text("hoge".to_string());

                    if socket.send(message).await.is_err() {
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
