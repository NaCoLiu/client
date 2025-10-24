use reqwest::{Client, Response, Error};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// HTTP 请求客户端封装
pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    /// 创建新的 HTTP 客户端
    pub fn new() -> Result<Self, Error> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()?;
        
        Ok(Self { client })
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

    /// GET 请求并解析 JSON
    pub async fn get_json<T: for<'de> Deserialize<'de>>(&self, url: &str) -> Result<T, Error> {
        self.client
            .get(url)
            .send()
            .await?
            .json::<T>()
            .await
    }

    /// POST 请求（JSON body）
    pub async fn post_json<T: Serialize>(&self, url: &str, body: &T) -> Result<Response, Error> {
        self.client
            .post(url)
            .json(body)
            .send()
            .await
    }

    /// POST 请求并解析 JSON 响应
    pub async fn post_json_response<T: Serialize, R: for<'de> Deserialize<'de>>(
        &self,
        url: &str,
        body: &T,
    ) -> Result<R, Error> {
        self.client
            .post(url)
            .json(body)
            .send()
            .await?
            .json::<R>()
            .await
    }

    /// PUT 请求（JSON body）
    pub async fn put_json<T: Serialize>(&self, url: &str, body: &T) -> Result<Response, Error> {
        self.client
            .put(url)
            .json(body)
            .send()
            .await
    }

    /// DELETE 请求
    pub async fn delete(&self, url: &str) -> Result<Response, Error> {
        self.client.delete(url).send().await
    }

    /// PATCH 请求（JSON body）
    pub async fn patch_json<T: Serialize>(&self, url: &str, body: &T) -> Result<Response, Error> {
        self.client
            .patch(url)
            .json(body)
            .send()
            .await
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new().expect("Failed to create HTTP client")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_request() {
        let client = HttpClient::new().unwrap();
        let result = client.get("https://httpbin.org/get").await;
        assert!(result.is_ok());
    }
}
