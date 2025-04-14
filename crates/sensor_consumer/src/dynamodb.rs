use aws_sdk_dynamodb::Client;

pub fn build_client<'a>(
) -> std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<Client>> + Send + 'a>> {
    Box::pin(async move {
        let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
            .test_credentials()
            .endpoint_url("http://localhost:8000")
            .load()
            .await;
        let dynamodb_local = aws_sdk_dynamodb::config::Builder::from(&config).build();
        let client = aws_sdk_dynamodb::Client::from_conf(dynamodb_local);

        // test code
        // let list_resp = client.list_tables().send().await;
        // match list_resp {
        //     Ok(res) => {
        //         println!("Found {} tables", res.table_names().len());
        //         for name in res.table_names() {
        //             println!("{}", name)
        //         }
        //     }
        //     Err(err) => eprintln!("Failed to list local tables {:?}", err.to_string()),
        // }
        Ok(client)
    })
}
