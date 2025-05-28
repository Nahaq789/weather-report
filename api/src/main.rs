use std::time::Duration;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::any,
    Router,
};
use chrono::Local;
use dynamodb::build_client;
use repository::SensorRepositoryImpl;
use sensor::{
    repository::SensorRepository,
    sensor::{status::Status, Sensor},
};
use sensor_dto::SensorDto;
use tokio::net::TcpListener;

pub mod dynamodb;
pub mod repository;
pub mod sensor_dto;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_ansi(false).init();
    tracing::info!("server start!!!");

    let listener = TcpListener::bind("0.0.0.0:5678").await?;

    let app = Router::new()
        .route("/sensor", any(sensor_handler))
        .route("/hoge", any(ws_hoge_handler));

    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn sensor_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket))
}

async fn ws_hoge_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_hoge(socket))
}

async fn handle_hoge(mut socket: WebSocket) {
    while let Some(_) = socket.recv().await {
        for _ in 0..1000 {
            #[cfg(feature = "not_docker")]
            {
                let result = SensorDto::generate();
                let msg = Message::Text(serde_json::to_string::<SensorDto>(&result).unwrap());
                let _ = socket.send(msg).await;
            }
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }
}

async fn handle_socket(mut socket: WebSocket) {
    println!("connection started");
    let client = match build_client().await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("dynamodb client error: {:?}", e);
            return;
        }
    };
    let repository = SensorRepositoryImpl::new(client);

    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            match msg {
                Message::Text(text) => loop {
                    tracing::info!("received message: {:?}", text);
                    let result = repository.get_sensor_data(&text).await.unwrap();

                    if result.is_empty() {
                        tracing::info!("the area data not fount: {}", text);
                        return;
                    }

                    let mut temperature_vec: Vec<f64> = Vec::new();
                    let mut humidity_vec: Vec<f64> = Vec::new();

                    for sensor in &result {
                        temperature_vec.push(sensor.measurements().temperature().value());
                        humidity_vec.push(sensor.measurements().humidity().value());
                    }
                    let aggregate = sensor_dto::Aggregate::build(temperature_vec, humidity_vec);

                    let errors = result
                        .iter()
                        .filter(|v| *v.status() == Status::Error)
                        .cloned()
                        .collect::<Vec<Sensor>>();
                    let time_stamp = Local::now();

                    if !errors.is_empty() {
                        let last_error = errors.last().unwrap();
                        let status = sensor_dto::Status::new(
                            errors.len(),
                            last_error.time_stamp().clone(),
                            errors,
                        );

                        let sensor = sensor_dto::SensorDto::build(
                            text.to_string(),
                            time_stamp,
                            aggregate,
                            Some(status),
                        );
                        match serde_json::to_string::<SensorDto>(&sensor) {
                            Ok(s) => {
                                let message = Message::Text(s);
                                if socket.send(message).await.is_err() {
                                    break;
                                }
                            }
                            Err(e) => println!("{:?}", e),
                        }
                        return;
                    }

                    let sensor =
                        sensor_dto::SensorDto::build(text.to_string(), time_stamp, aggregate, None);
                    match serde_json::to_string(&sensor) {
                        Ok(s) => {
                            let message = Message::Text(s);
                            tracing::info!("{:?}", message);
                            if socket.send(message).await.is_err() {
                                break;
                            }
                        }
                        Err(e) => println!("{:?}", e),
                    }

                    tokio::time::sleep(Duration::from_secs(5)).await
                },
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
