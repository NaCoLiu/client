use serde_json::Value as JsonValue;
use std::fs;
use std::path::PathBuf;
use std::env;

/// 获取用户数据文件路径（存储在应用目录下）
fn get_data_path() -> Result<PathBuf, String> {
    // 获取当前可执行文件所在目录
    let exe_path = env::current_exe()
        .map_err(|e| format!("无法获取可执行文件路径: {}", e))?;
    
    let exe_dir = exe_path.parent()
        .ok_or("无法获取可执行文件目录")?;
    
    // 在 dev 模式下，使用项目根目录
    // 在 build 模式下，使用可执行文件目录
    let app_dir = if cfg!(debug_assertions) {
        // Dev 模式：使用项目根目录
        let mut current = exe_dir.to_path_buf();
        // 向上查找，直到找到包含 src-tauri 的目录
        while !current.join("src-tauri").exists() && current.parent().is_some() {
            current = current.parent().unwrap().to_path_buf();
        }
        current
    } else {
        // Build 模式：使用可执行文件所在目录
        exe_dir.to_path_buf()
    };
    
    // 确保目录存在
    if !app_dir.exists() {
        fs::create_dir_all(&app_dir)
            .map_err(|e| format!("创建应用数据目录失败: {}", e))?;
    }
    
    Ok(app_dir.join("user_data.yml"))
}

/// 从 YAML 文件读取数据，返回 JSON 格式
pub fn load_yaml() -> Result<JsonValue, String> {
    let path = get_data_path()?;
    
    if !path.exists() {
        // 如果文件不存在，返回空对象
        return Ok(JsonValue::Object(serde_json::Map::new()));
    }

    let content = fs::read_to_string(&path)
        .map_err(|e| format!("读取用户数据文件失败: {}", e))?;
    
    // 将 YAML 转换为 JSON
    let yaml_value: serde_yaml::Value = serde_yaml::from_str(&content)
        .map_err(|e| format!("解析 YAML 失败: {}", e))?;
    
    let json_value = serde_json::to_value(yaml_value)
        .map_err(|e| format!("转换为 JSON 失败: {}", e))?;
    
    Ok(json_value)
}

/// 将 JSON 数据保存为 YAML 文件
pub fn save_yaml(data: JsonValue) -> Result<(), String> {
    let path = get_data_path()?;
    
    // 将 JSON 转换为 YAML
    let yaml_value: serde_yaml::Value = serde_json::from_value(data)
        .map_err(|e| format!("转换 JSON 数据失败: {}", e))?;
    
    let content = serde_yaml::to_string(&yaml_value)
        .map_err(|e| format!("序列化为 YAML 失败: {}", e))?;
    
    fs::write(&path, content)
        .map_err(|e| format!("保存用户数据文件失败: {}", e))?;
    
    Ok(())
}
