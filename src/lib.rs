use std::fs::File;
use std::io::Read;

use reqwest::Client;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PostRequest {
    pub channel: String,
    pub instruction: String,
    pub data: Option<Value>,
}

impl PostRequest {
    pub fn new(channel: &str, instruction: &str, data: Option<Value>) -> Self {
        Self {
            channel: channel.to_string(),
            instruction: instruction.to_string(),
            data,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PostResponse {
    pub success: bool,
    pub msg: Option<String>,
    pub data: Option<Value>,
}


pub struct ServerApi {
    url: String
}

impl ServerApi {
    pub async fn new(server_url: &str) -> Self {
        Self {
            url: server_url.to_string(),
        }
    }

    pub async fn post(&self, post_request: PostRequest) -> Result<PostResponse, anyhow::Error> {
        let msg = serde_json::to_value(post_request)?;
    
        let client = Client::new();
    
        let res = client.post(&self.url)
            .header("Content-Type", "application/json")
            .body(msg.to_string())
            .send()
            .await?;
    
        if res.status().is_success() {
            let body = res.text().await?;
            let response: PostResponse = serde_json::from_str(&body).unwrap();
            Ok(response)
        } else {
            Err(anyhow::anyhow!("Request failed with status: {}", res.status()))
        }
    }
    
    
    pub async fn put_to_s3(file_path: &str, url: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Open the file in read-only mode.
        let mut file = File::open(file_path)?;
    
        // Read the file's contents into a byte vector.
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
    
        // Create a reqwest Client.
        let client = Client::new();
    
        // Send a PUT request.
        let response = client.put(url)
            .body(buffer)
            .send()
            .await?;
    
        // Ensure the request was successful.
        if !response.status().is_success() {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to upload to S3")));
        }
    
        Ok(())
    }
}

