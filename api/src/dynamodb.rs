use std::env;

use aws_sdk_dynamodb::Client;

pub async fn build_client() -> anyhow::Result<Client> {
    let endpoint_url =
        env::var("DYNAMO_ENDPOINT").unwrap_or_else(|_| "http://localhost:8000".to_string());
    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .test_credentials()
        .endpoint_url(&endpoint_url)
        .load()
        .await;
    let dynamodb_local = aws_sdk_dynamodb::config::Builder::from(&config).build();
    let client = aws_sdk_dynamodb::Client::from_conf(dynamodb_local);

    Ok(client)
}
