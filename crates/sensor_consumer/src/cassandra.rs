use cassandra_cpp::{Cluster, Session};

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

// TODO
pub fn save_sensor<'a>(
    session: &'a Session,
    data: String,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send + '_>> {
    Box::pin(async move { Ok(()) })
}
