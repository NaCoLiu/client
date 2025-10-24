use crate::config;
use crate::http;
use crate::logger::{console_log, LogType};
use crate::login::lib::get_screen_hwid;
use serde::{Deserialize, Serialize};

// 卡密验证请求结构
#[derive(Serialize)]
struct CardVerifyRequest {
    key: String,
    hwid: String,
}

// 卡密信息结构
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct CardInfo {
    id: i64,
    key: String,
    status: String, // used(使用中), unused(未使用), expired(过期)
    #[serde(default)]
    description: Option<String>,
    #[serde(rename = "usedAt")]
    used_at: Option<String>,
    #[serde(rename = "expiredAt")]
    expired_at: Option<String>,
    hwid: Option<String>,
    #[serde(rename = "bindAt")]
    bind_at: Option<String>,
}

// 卡密验证响应结构
#[derive(Deserialize, Debug)]
struct CardVerifyResponse {
    #[serde(default)]
    success: bool,
    #[serde(default)]
    error: Option<String>,
    #[serde(default)]
    card: Option<CardInfo>,
    #[serde(rename = "serverTime")]
    server_time: Option<i64>, // 服务器时间戳，用于防止客户端时间篡改
}

pub async fn request_connection_server() -> bool {
    let client = http::HttpClient::new().unwrap();
    let backend_url = config::get_backend_url();
    let url = format!("{}/api/cards", backend_url);

    let response = client.get(&url).await;

    match response {
        Ok(_) => true,
        Err(_) => {
            console_log(LogType::Failure, "服务器连接失败");
           
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            #[cfg(not(debug_assertions))]
            std::process::exit(1);

            #[cfg(debug_assertions)]
            false
        }
    }
}

pub async fn login_request(password: &str) -> Result<(bool, i64), String> {
    if password.is_empty() {
        return Err("卡密不能为空".to_string());
    }

    let hwid = get_screen_hwid().await;
    if hwid.is_empty() {
        return Err("无法获取硬件ID".to_string());
    }

    let client = http::HttpClient::new().map_err(|e| format!("创建HTTP客户端失败: {}", e))?;

    let backend_url = config::get_backend_url();
    let verify_url = format!("{}/api/cards/verify", backend_url);

    let request_body = CardVerifyRequest {
        key: password.to_string(),
        hwid: hwid.clone(),
    };

    let http_response = client
        .post_json(&verify_url, &request_body)
        .await
        .map_err(|e| format!("网络请求失败: {}", e))?;

    let response_text = http_response
        .text()
        .await
        .map_err(|e| format!("读取响应失败: {}", e))?;

    let response: CardVerifyResponse =
        serde_json::from_str(&response_text).map_err(|e| format!("响应解析失败: {}", e))?;

    if !response.success {
        let error_msg = response.error.unwrap_or_else(|| "验证失败".to_string());
        console_log(LogType::Failure, &format!("✗ {}", error_msg));
        return Err(error_msg);
    }

    let card = response.card.ok_or("服务器未返回卡密信息")?;

    // 检查卡密状态
    if card.status != "used" {
        let error_msg = match card.status.as_str() {
            "unused" => "卡密未使用",
            "expired" => "卡密已过期",
            _ => "卡密状态无效",
        };

        return Err(error_msg.to_string());
    }

    let expired_at_str: String = card.expired_at.ok_or("服务器未返回过期时间")?;
    let expired_at =
        chrono::DateTime::parse_from_rfc3339(&expired_at_str).map_err(|_| "解析过期时间失败")?;

    let expiration_timestamp = expired_at.timestamp();

    check_expiration(expiration_timestamp, response.server_time)?;

    Ok((true, expiration_timestamp))
}

/// 检查时间戳是否过期
/// 优先使用服务器时间，防止客户端时间篡改
pub fn check_expiration(
    expiration_timestamp: i64,
    server_time: Option<i64>,
) -> Result<i64, String> {
    let current_timestamp = server_time.unwrap_or_else(|| chrono::Utc::now().timestamp());

    if current_timestamp >= expiration_timestamp {
        console_log(LogType::Failure, "会话已过期");
        Err("会话已过期，请重新登录".to_string())
    } else {
        // console_log(LogType::Success, "时间戳验证通过");
        Ok(expiration_timestamp)
    }
}
