#[cfg(test)]
mod tests {
    use cnctd_server_api::{ServerApi, PostRequest};

    #[tokio::test]
    async fn test_commands() {
        let url = dotenv::var("TEST_SERVER").unwrap();
        let server_api = ServerApi::new(&url).await;
        let request = PostRequest::new("test", "test", None);
        let response = server_api.post(request).await.unwrap();
        println!("response: {:?}", response);
    }
}

