use sensor_simulator::{
    kafka::producer::{build_producer, send_message},
    sensor::{location::area::Area, season::Season, Sensor},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let producer = build_producer();
    let area = Area::Tokyo;
    let season = Season::Spring;
    let sensor = Sensor::new(&area, &season);
    let _ = send_message(&producer, sensor).await;
    Ok(())
}
