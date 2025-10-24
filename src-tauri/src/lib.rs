// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

pub mod config;
pub mod http;
pub mod logger;
pub mod login;
pub mod user_data;

use serde_json::{json, Value as JsonValue};
use tauri::Manager;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::login::request::{login_request, check_expiration};
use crate::logger::{console_log, LogType};

// Tauri 命令：读取 YAML 数据
#[tauri::command]
fn load_user_data() -> Result<JsonValue, String> {
    user_data::load_yaml()
}

// Tauri 命令：保存数据到 YAML
#[tauri::command]
fn save_user_data(data: JsonValue) -> Result<(), String> {
    user_data::save_yaml(data)
}

#[tauri::command]
fn get_app_info() -> JsonValue {
    let app = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let info = json!({
        "app": app,
        "version": version,
    });
    info
}
#[tauri::command]
async fn login(password: &str) -> Result<(bool, i64), String> {
    login_request(password).await
}

#[tauri::command]
fn check_session_expiration(expiration_timestamp: i64) -> Result<i64, String> {
    check_expiration(expiration_timestamp)
}

#[tauri::command]
fn start_session_monitor(expiration_timestamp: i64) {
    console_log(LogType::WARNING, "启动会话监控任务，每10秒检查一次");
    
    let shared_timestamp = Arc::new(Mutex::new(expiration_timestamp));
    
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(10)); // 10秒（测试用）
        
        loop {
            interval.tick().await;
            
            let timestamp = {
                let lock = shared_timestamp.lock().unwrap();
                *lock
            };
            
            console_log(LogType::WARNING, "正在检查会话是否过期...");
            
            match check_expiration(timestamp) {
                Ok(_) => {
                    console_log(LogType::SUCCESS, "会话仍然有效");
                }
                Err(_) => {
                    console_log(LogType::FAILURE, "会话已过期，程序将退出");
                    std::process::exit(1);
                }
            }
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            load_user_data,
            save_user_data,
            get_app_info,
            login,
            check_session_expiration,
            start_session_monitor
        ])
        .setup(|app| {
            // 设置窗口标题为包名
            let window_title = env!("CARGO_PKG_NAME");
            // 获取所有窗口并设置标题
            for (_, window) in app.webview_windows() {
                let _ = window.set_title(window_title);
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
