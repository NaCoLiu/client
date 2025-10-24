// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

pub mod config;
pub mod http;
pub mod logger;
pub mod login;
pub mod user_data;

use serde_json::{json, Value as JsonValue};
use tauri::Manager;
use std::time::Duration;

use crate::login::request::{login_request};
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
fn start_session_monitor(password: String) {
    console_log(LogType::WARNING, "启动会话监控任务，每10秒向后端验证一次");
    
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(10));
        
        loop {
            interval.tick().await;
            
            console_log(LogType::WARNING, "正在向后端验证会话...");
            
            match login_request(&password).await {
                Ok((success, _expiration_timestamp)) => {
                    if success {
                        console_log(LogType::SUCCESS, "后端验证通过，会话仍然有效");
                    } else {
                        console_log(LogType::FAILURE, "后端验证失败，程序将退出");
                        std::process::exit(1);
                    }
                }
                Err(e) => {
                    console_log(LogType::FAILURE, &format!("后端验证请求失败: {}，程序将退出", e));
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
