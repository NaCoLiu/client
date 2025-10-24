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
    status: String,  // used(使用中), unused(未使用), expired(过期)
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
    message: String,
    #[serde(default)]
    error: Option<String>,
    #[serde(default)]
    card: Option<CardInfo>,
}

pub async fn request_connection_server() -> bool {
    console_log(
        LogType::WARNING,
        &format!("Requesting connection to the server..."),
    );
    let client = http::HttpClient::new().unwrap();
    let backend_url = config::get_backend_url();
    let url = format!("{}/api", backend_url);

    let response = client.get(&url).await;

    // tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    match response {
        Ok(_) => {
            console_log(
                LogType::SUCCESS,
                &format!("Successfully connected to the server."),
            );
            true
        }
        Err(_) => {
            console_log(
                LogType::FAILURE,
                &format!("Connection server request failed."),
            );
            #[cfg(not(debug_assertions))]
            std::process::exit(1);

            #[cfg(debug_assertions)]
            false
        }
    }
}

pub async fn login_request(password: &str) -> Result<(bool, i64), String> {
    console_log(LogType::WARNING, "开始卡密验证...");
    
    if password.is_empty() {
        return Err("卡密不能为空".to_string());
    }

    // 获取硬件ID
    let hwid = get_screen_hwid().await;
    if hwid.is_empty() {
        return Err("无法获取硬件ID".to_string());
    }
    
    console_log(LogType::WARNING, &format!("硬件ID(MD5): {}", hwid));

    let client = http::HttpClient::new()
        .map_err(|e| format!("创建HTTP客户端失败: {}", e))?;

    let backend_url = config::get_backend_url();
    let verify_url = format!("{}/api/cards/verify", backend_url);
    console_log(LogType::WARNING, &format!("验证地址: {}", verify_url));

    let request_body = CardVerifyRequest {
        key: password.to_string(),
        hwid: hwid.clone(),
    };

    // 发送验证请求并解析响应
    let http_response = client.post_json(&verify_url, &request_body).await
        .map_err(|e| format!("网络请求失败: {}", e))?;
    
    let response_text = http_response.text().await
        .map_err(|e| format!("读取响应失败: {}", e))?;
    
    console_log(LogType::WARNING, &format!("服务器响应: {}", response_text));
    
    let response: CardVerifyResponse = serde_json::from_str(&response_text)
        .map_err(|e| format!("响应解析失败: {}", e))?;

    if !response.success {
        let error_msg = response.error.unwrap_or_else(|| "验证失败".to_string());
        console_log(LogType::FAILURE, &format!("✗ {}", error_msg));
        return Err(error_msg);
    }

    console_log(LogType::SUCCESS, &format!("✓ {}", response.message));
    
    let card = response.card.ok_or("服务器未返回卡密信息")?;
    
    // 检查卡密状态
    console_log(LogType::WARNING, &format!("卡密状态: {}", card.status));
    if card.status != "used" {
        let error_msg = match card.status.as_str() {
            "unused" => "卡密未使用",
            "expired" => "卡密已过期",
            _ => "卡密状态无效"
        };
        console_log(LogType::FAILURE, error_msg);
        return Err(error_msg.to_string());
    }
    console_log(LogType::SUCCESS, "卡密状态正常（使用中）");
    
    // 检查过期时间
    let expired_at_str = card.expired_at.ok_or("服务器未返回过期时间")?;
    let expired_at = chrono::DateTime::parse_from_rfc3339(&expired_at_str)
        .map_err(|_| "解析过期时间失败")?;
    
    let expiration_timestamp = expired_at.timestamp();
    console_log(
        LogType::SUCCESS,
        &format!("过期时间: {} (UTC时间戳: {})", expired_at_str, expiration_timestamp),
    );
    
    // 检查是否已过期
    check_expiration(expiration_timestamp)?;
    
    Ok((true, expiration_timestamp))
}

/// 检查时间戳是否过期，如果未过期返回时间戳，如果已过期返回错误
pub fn check_expiration(expiration_timestamp: i64) -> Result<i64, String> {
    // 使用真实当前时间进行对比
    let current_timestamp = chrono::Utc::now().timestamp();
    
    if current_timestamp >= expiration_timestamp {
        console_log(LogType::FAILURE, "会话已过期");
        Err("会话已过期，请重新登录".to_string())
    } else {
        let remaining = expiration_timestamp - current_timestamp;
        console_log(LogType::SUCCESS, &format!("会话有效，剩余时间: {} 秒", remaining));
        Ok(expiration_timestamp)
    }
}
