use crate::config;
use crate::http;
use crate::logger::{console_log, LogType};
use crate::login::lib::get_screen_hwid;
use serde::{Deserialize, Serialize};

// 后端登录请求和响应结构（暂时未使用，等待后端API实现）
#[allow(dead_code)]
#[derive(Serialize)]
struct LoginRequest {
    password: String,
    hwid: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct LoginResponse {
    success: bool,
    #[serde(default)]
    message: String,
}

pub async fn request_connection_server() -> bool {
    console_log(
        LogType::WARNING,
        &format!("Requesting connection to the server..."),
    );
    let client = http::HttpClient::new().unwrap();
    let backend_url = config::get_backend_url();
    let url = format!("{}/todos/1", backend_url);

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
    if password.is_empty() {
        console_log(LogType::FAILURE, "密钥不能为空");
        return Err("密钥不能为空".to_string());
    }

    // 获取硬件ID
    let hwid = get_screen_hwid().await;
    if hwid.is_empty() {
        console_log(LogType::WARNING, "无法获取硬件ID，继续登录...");
        // 返回错误以阻止登录
        return Err("无法获取硬件ID".to_string());
    } else {
        console_log(LogType::WARNING, &format!("硬件ID: {}", hwid));
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    console_log(LogType::SUCCESS, "登录成功（模拟）");

    // 使用真实时间计算过期时间（当前时间 + 10秒，用于测试）
    let current_time = chrono::Utc::now().timestamp();
    let expiration_timestamp = current_time + 10;
    
    console_log(
        LogType::SUCCESS,
        &format!("到期时间(UTC): {} (登录后10秒过期)", expiration_timestamp),
    );

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

// 真实后端登录代码（暂时注释）：
/*
let client = http::HttpClient::new().map_err(|e| {
    console_log(LogType::FAILURE, &format!("创建HTTP客户端失败: {}", e));
    format!("创建HTTP客户端失败: {}", e)
})?;

let backend_url = config::get_backend_url();
let login_url = format!("{}/api/auth/login", backend_url);

let request_body: LoginRequest = LoginRequest {
    password: password.to_string(),
    hwid,
};

match client.post_json_response::<LoginRequest, LoginResponse>(&login_url, &request_body).await {
    Ok(response) => {
        if response.success {
            console_log(LogType::SUCCESS, "登录成功");
            Ok(true)
        } else {
            let message = if response.message.is_empty() {
                "登录失败"
            } else {
                &response.message
            };
            console_log(LogType::FAILURE, &format!("登录失败: {}", message));
            Err(message.to_string())
        }
    }
    Err(e) => {
        console_log(LogType::FAILURE, &format!("登录请求失败: {}", e));
        Err(format!("登录请求失败: {}", e))
    }
}
*/
