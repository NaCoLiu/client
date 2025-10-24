use once_cell::sync::Lazy;
use std::sync::RwLock;

pub struct AppConfig {
    pub backend_url: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            // 默认后端地址
            backend_url: "https://jsonplaceholder.typicode.com".to_string(),
        }
    }
}

// 全局配置实例
pub static CONFIG: Lazy<RwLock<AppConfig>> = Lazy::new(|| RwLock::new(AppConfig::default()));

// 获取后端地址
pub fn get_backend_url() -> String {
    CONFIG.read().unwrap().backend_url.clone()
}

// 设置后端地址
pub fn set_backend_url(url: String) {
    CONFIG.write().unwrap().backend_url = url;
}
