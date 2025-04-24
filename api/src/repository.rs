use aws_sdk_dynamodb::Client;

#[derive(Debug)]
pub struct SensorRepositoryImpl {
    client: Client,
}
