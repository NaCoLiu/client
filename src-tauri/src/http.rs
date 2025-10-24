use reqwest::{Client, Error, Response};
use serde::Serialize;
use std::time::Duration;

/// HTTP 请求客户端封装
pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    /// 默认超时时间（秒）
    const DEFAULT_TIMEOUT: u64 = 30;

    /// 创建新的 HTTP 客户端（使用默认超时）
    pub fn new() -> Result<Self, Error> {
        Self::with_timeout(Self::DEFAULT_TIMEOUT)
    }

    /// 创建带自定义超时的客户端
    pub fn with_timeout(timeout_secs: u64) -> Result<Self, Error> {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .build()?;

        Ok(Self { client })
    }

    /// GET 请求
    pub async fn get(&self, url: &str) -> Result<Response, Error> {
        self.client.get(url).send().await
    }

    /// POST 请求（JSON body）
    pub async fn post_json<T: Serialize>(&self, url: &str, body: &T) -> Result<Response, Error> {
        self.client.post(url).json(body).send().await
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new().expect("Failed to create HTTP client")
    }
}
