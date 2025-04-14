#[cfg(feature = "with_cassandra")]
use cassandra_cpp::{BindRustType, Cluster, Session, Statement};

use crate::sensor::Sensor;

#[cfg(feature = "with_cassandra")]
pub fn connect_cluster() -> std::pin::Pin<
    Box<dyn std::future::Future<Output = Result<Session, cassandra_cpp::Error>> + Send>,
> {
    let host = "localhost";
    let user = "user";
    let password = "password";

    let mut cluster = Cluster::default();
    let _ = cluster.set_credentials(user, password);
    let _ = cluster.set_contact_points(host);

    Box::pin(async move { cluster.connect_async().await })
}

#[cfg(feature = "with_cassandra")]
pub fn save_sensor<'a>(
    session: &'a Session,
    data: &'a str,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send + 'a>> {
    println!("start save");
    Box::pin(async move {
        println!("start serde");
        let sensor = Sensor::from_str(data)?;

        let query = "INSERT INTO sensor_key_space.sensor (sensor_id, time_stamp, area, latitude, longitude, temperature, humidity, status) VALUES (?, ?, ?, ?, ?, ?, ?, ?);";

        let mut statement = Statement::new(query, 8);
        println!("start statement");
        map_err(
            statement.bind(0, sensor.sensor_id.as_str()),
            "Failed to bind sensor_id",
        )?;
        map_err(
            statement.bind(1, sensor.time_stamp.to_rfc3339().as_str()),
            "Failed to bind time_stamp",
        )?;
        map_err(
            statement.bind(2, sensor.area.as_str()),
            "Failed to bind area",
        )?;
        map_err(
            statement.bind(3, sensor.latitude),
            "Failed to bind latitude",
        )?;
        map_err(
            statement.bind(4, sensor.longitude),
            "Failed to bind longitude",
        )?;
        map_err(
            statement.bind(5, sensor.temperature),
            "Failed to bind temperature",
        )?;
        map_err(
            statement.bind(6, sensor.humidity),
            "Failed to bind humidity",
        )?;
        map_err(
            statement.bind(7, sensor.status.as_str()),
            "Failed to bind status",
        )?;
        let result = session.execute(&statement).await;
        map_err(result, "Failed to execute query")?;
        Ok(())
    })
}

fn map_err<T, E>(result: Result<T, E>, msg: &str) -> anyhow::Result<T>
where
    E: std::fmt::Display,
{
    match result {
        Ok(val) => Ok(val),
        Err(e) => Err(anyhow::anyhow!("{}: {}", msg, e)),
    }
}
