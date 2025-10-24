use once_cell::sync::Lazy;

/// 应用配置结构
pub struct AppConfig {
    pub backend_url: String,
}

impl AppConfig {
    /// 获取后端 URL（如果未设置则返回默认值）
    fn backend_url(&self) -> &str {
        if self.backend_url.is_empty() {
            "http://localhost:3000"
        } else {
            &self.backend_url
        }
    }
}

/// 全局配置实例（只读，通过环境变量或配置文件初始化）
static CONFIG: Lazy<AppConfig> = Lazy::new(|| {
    // 可以从环境变量或配置文件读取
    AppConfig {
        backend_url: std::env::var("BACKEND_URL")
            .unwrap_or_else(|_| "http://localhost:3000".to_string()),
    }
});

/// 获取后端地址
pub fn get_backend_url() -> String {
    CONFIG.backend_url().to_string()
}
