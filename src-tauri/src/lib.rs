// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

pub mod config;
pub mod http;
pub mod logger;
pub mod login;
pub mod data;

use serde_json::{json, Value as JsonValue};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::Manager;
use tokio::sync::broadcast;

use crate::logger::{console_log, LogType};
use crate::login::request::login_request;

// 全局登录状态管理
struct LoginState {
    is_logged_in: bool,
    user_info: Option<JsonValue>,
    shutdown_sender: Option<broadcast::Sender<()>>,
}

impl LoginState {
    fn new() -> Self {
        Self {
            is_logged_in: false,
            user_info: None,
            shutdown_sender: None,
        }
    }
}

type AppState = Arc<Mutex<LoginState>>;

// Tauri 命令：读取 YAML 数据
#[tauri::command]
fn load_data() -> Result<JsonValue, String> {
    data::load_yaml()
}

// Tauri 命令：保存数据到 YAML
#[tauri::command]
fn save_data(data: JsonValue) -> Result<(), String> {
    data::save_yaml(data)
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
async fn login(password: &str, app_state: tauri::State<'_, AppState>) -> Result<(bool, i64), String> {
    let result = login_request(password).await;
    
    // 如果登录成功，更新全局状态并启动监控
    if let Ok((success, timestamp)) = &result {
        if *success {
            // 先更新状态
            {
                let mut state = app_state.lock().unwrap();
                state.is_logged_in = true;
                state.user_info = Some(json!({
                    "password": password,
                    "expiration_timestamp": timestamp,
                    "login_time": chrono::Utc::now().timestamp()
                }));
            } // 释放锁
            
            // 然后启动监控（在锁释放后）
            start_monitoring(password.to_string(), app_state.inner().clone());
        }
    }
    
    result
}

// 获取登录状态
#[tauri::command]
fn get_login_status(app_state: tauri::State<'_, AppState>) -> Result<JsonValue, String> {
    let state = app_state.lock().unwrap();
    Ok(json!({
        "is_logged_in": state.is_logged_in,
        "user_info": state.user_info
    }))
}

// 设置登录状态
#[tauri::command]
fn set_login_status(
    #[allow(non_snake_case)] isLoggedIn: bool, 
    #[allow(non_snake_case)] userInfo: Option<JsonValue>,
    app_state: tauri::State<'_, AppState>
) -> Result<(), String> {
    let mut state = app_state.lock().unwrap();
    state.is_logged_in = isLoggedIn;
    state.user_info = userInfo;
    Ok(())
}

// 退出登录
#[tauri::command]
async fn logout(app_state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut state = app_state.lock().unwrap();
    
    // 停止监控任务
    if let Some(sender) = &state.shutdown_sender {
        let _ = sender.send(());
    }
    
    state.is_logged_in = false;
    state.user_info = None;
    state.shutdown_sender = None;
    Ok(())
}




// 启动会话监控（内部函数）
fn start_monitoring(password: String, app_state: AppState) {
    let (shutdown_sender, mut shutdown_receiver) = broadcast::channel(1);
    
    // 保存shutdown_sender到状态中
    {
        let mut state = app_state.lock().unwrap();
        state.shutdown_sender = Some(shutdown_sender);
    }
    
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(10));

        loop {
            tokio::select! {
                _ = interval.tick() => {
                    // 检查登录状态是否仍然有效
                    let should_continue = {
                        let state = app_state.lock().unwrap();
                        state.is_logged_in
                    };
                    
                    if !should_continue {
                        console_log(LogType::Warning, "用户已退出登录，停止会话监控");
                        break;
                    }
                    
                    match login_request(&password).await {
                        Ok((success, _expiration_timestamp)) => {
                            if !success {
                                console_log(LogType::Failure, "会话验证失败，程序将退出");
                                std::process::exit(1);
                            }
                        }
                        Err(e) => {
                            console_log(
                                LogType::Failure,
                                &format!("后端验证请求失败: {}，程序将退出", e),
                            );
                            std::process::exit(1);
                        }
                    }
                }
                _ = shutdown_receiver.recv() => {
                    console_log(LogType::Success, "收到退出信号，停止会话监控");
                    break;
                }
            }
        }
    });
}

// 兼容性函数：为了保持与前端的兼容性
#[tauri::command]
fn start_session_monitor(password: String, app_state: tauri::State<'_, AppState>) {
    // 检查是否已经在监控中
    let already_monitoring = {
        let state = app_state.lock().unwrap();
        state.shutdown_sender.is_some()
    };
    
    if !already_monitoring {
        start_monitoring(password, app_state.inner().clone());
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let login_state = Arc::new(Mutex::new(LoginState::new()));
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(login_state)
        .invoke_handler(tauri::generate_handler![
            load_data,
            save_data,
            get_app_info,
            login,
            start_session_monitor,
            get_login_status,
            set_login_status,
            logout
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
