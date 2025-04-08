use cassandra_cpp::{BindRustType, Cluster, Session, Statement};

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

pub fn save_sensor<'a>(
    session: &'a Session,
    data: &'a str,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send + 'a>> {
    Box::pin(async move {
        let query = "INSERT INTO sensor_key_space.sensor (sensor_id, time_stamp, area, latitude, longitude, temperature, humidity, status) VALUES (?, ?, ?, ?, ?, ?, ?, ?);";
        let mut statement = Statement::new(query, 8);
        map_err(statement.bind(0, data), "Failed to bind sensor_id")?;
        map_err(statement.bind(1, data), "Failed to bind time_stamp")?;
        map_err(statement.bind(2, data), "Failed to bind area")?;
        map_err(statement.bind(3, data), "Failed to bind latitude")?;
        map_err(statement.bind(4, data), "Failed to bind longitude")?;
        map_err(statement.bind(5, data), "Failed to bind temperature")?;
        map_err(statement.bind(6, data), "Failed to bind humidity")?;
        map_err(statement.bind(7, data), "Failed to bind status")?;
        map_err(session.execute(&statement).await, "Failed to execute quest")?;
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
