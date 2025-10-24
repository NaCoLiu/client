use serde_json::Value as JsonValue;
use std::env;
use std::fs;
use std::path::PathBuf;

/// 获取用户数据文件路径
fn get_data_path() -> Result<PathBuf, String> {
    let exe_path = env::current_exe().map_err(|e| format!("无法获取可执行文件路径: {}", e))?;

    let exe_dir = exe_path
        .parent()
        .ok_or("无法获取可执行文件目录")?;

    // 开发模式：使用项目根目录，生产模式：使用可执行文件目录
    let app_dir = if cfg!(debug_assertions) {
        find_project_root(exe_dir)?
    } else {
        exe_dir.to_path_buf()
    };

    Ok(app_dir.join("data.yml"))
}

/// 查找项目根目录（包含 src-tauri 的目录）
fn find_project_root(start_dir: &std::path::Path) -> Result<PathBuf, String> {
    let mut current = start_dir.to_path_buf();

    while !current.join("src-tauri").exists() {
        current = current
            .parent()
            .ok_or("未找到项目根目录")?
            .to_path_buf();
    }

    Ok(current)
}

/// 从 YAML 文件读取数据，返回 JSON 格式
pub fn load_yaml() -> Result<JsonValue, String> {
    let path = get_data_path()?;

    // 文件不存在时返回空对象
    if !path.exists() {
        return Ok(JsonValue::Object(serde_json::Map::new()));
    }

    let content = fs::read_to_string(&path).map_err(|e| format!("读取用户数据文件失败: {}", e))?;

    // YAML -> JSON 转换
    let yaml_value: serde_yaml::Value =
        serde_yaml::from_str(&content).map_err(|e| format!("解析 YAML 失败: {}", e))?;

    serde_json::to_value(yaml_value).map_err(|e| format!("转换为 JSON 失败: {}", e))
}

/// 将 JSON 数据保存为 YAML 文件
pub fn save_yaml(data: JsonValue) -> Result<(), String> {
    let path = get_data_path()?;

    // JSON -> YAML 转换
    let yaml_value: serde_yaml::Value =
        serde_json::from_value(data).map_err(|e| format!("转换 JSON 数据失败: {}", e))?;

    let content =
        serde_yaml::to_string(&yaml_value).map_err(|e| format!("序列化为 YAML 失败: {}", e))?;

    fs::write(&path, content).map_err(|e| format!("保存用户数据文件失败: {}", e))
}
