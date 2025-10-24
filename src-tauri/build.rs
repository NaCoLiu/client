use std::fmt;
use std::fs;
use std::path::PathBuf;

const PACKAGE_JSON: &str = "package.json";
const CARGO_TOML: &str = "Cargo.toml";
const TAURI_CONF: &str = "tauri.conf.json";
const SRC_TAURI: &str = "src-tauri";

fn main() {
    // 同步 package.json 到 Cargo.toml 和 tauri.conf.json
    let _ = sync_package_info();

    tauri_build::build()
}

fn sync_package_info() -> Result<(), SyncError> {
    let project_root = get_project_root()?;
    
    // 添加文件监控，当 package.json 改变时重新运行构建脚本
    let package_json_path = project_root.join(PACKAGE_JSON);
    println!("cargo:rerun-if-changed={}", package_json_path.display());
    
    let package_info = read_package_json(&project_root)?;
    
    sync_cargo_toml(&project_root, &package_info)?;
    sync_tauri_config(&project_root, &package_info)?;

    Ok(())
}

/// 获取项目根目录
fn get_project_root() -> Result<PathBuf, SyncError> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .map_err(|_| SyncError::EnvVar("CARGO_MANIFEST_DIR"))?;
    
    PathBuf::from(&manifest_dir)
        .parent()
        .ok_or(SyncError::InvalidPath("无法获取项目根目录"))
        .map(|p| p.to_path_buf())
}

/// 包信息结构
#[derive(Debug, Clone)]
struct PackageInfo {
    version: String,
    capitalized_name: String,
}

/// 读取并解析 package.json
fn read_package_json(project_root: &PathBuf) -> Result<PackageInfo, SyncError> {
    let package_json_path = project_root.join(PACKAGE_JSON);
    
    let content = fs::read_to_string(&package_json_path)
        .map_err(|e| SyncError::FileRead(PACKAGE_JSON, e))?;
    
    let json: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| SyncError::JsonParse(PACKAGE_JSON, e))?;

    let name = json["name"]
        .as_str()
        .ok_or(SyncError::MissingField("name", PACKAGE_JSON))?
        .to_string();
    
    let version = json["version"]
        .as_str()
        .ok_or(SyncError::MissingField("version", PACKAGE_JSON))?
        .to_string();

    let capitalized_name = capitalize_first(&name);

    Ok(PackageInfo {
        version,
        capitalized_name,
    })
}

/// 同步到 Cargo.toml
fn sync_cargo_toml(project_root: &PathBuf, info: &PackageInfo) -> Result<(), SyncError> {
    let cargo_toml_path = project_root.join(SRC_TAURI).join(CARGO_TOML);
    
    let content = fs::read_to_string(&cargo_toml_path)
        .map_err(|e| SyncError::FileRead(CARGO_TOML, e))?;
    
    let mut cargo_toml: toml::Value = toml::from_str(&content)
        .map_err(|e| SyncError::TomlParse(CARGO_TOML, e))?;

    let mut changes = Vec::new();
    
    if let Some(package) = cargo_toml.get_mut("package") {
        if let Some(table) = package.as_table_mut() {
            // 更新名称
            if !matches!(table.get("name").and_then(|v| v.as_str()), Some(name) if name == info.capitalized_name) {
                table.insert("name".to_string(), toml::Value::String(info.capitalized_name.clone()));
                changes.push(format!("name: {}", info.capitalized_name));
            }
            
            // 更新版本
            if !matches!(table.get("version").and_then(|v| v.as_str()), Some(ver) if ver == info.version) {
                table.insert("version".to_string(), toml::Value::String(info.version.clone()));
                changes.push(format!("version: {}", info.version));
            }
        }
    }

    if !changes.is_empty() {
        let new_content = toml::to_string_pretty(&cargo_toml)
            .map_err(|e| SyncError::TomlSerialize(CARGO_TOML, e))?;
        
        fs::write(&cargo_toml_path, new_content)
            .map_err(|e| SyncError::FileWrite(CARGO_TOML, e))?;
    }

    Ok(())
}

/// 同步到 tauri.conf.json
fn sync_tauri_config(project_root: &PathBuf, info: &PackageInfo) -> Result<(), SyncError> {
    let tauri_conf_path = project_root.join(SRC_TAURI).join(TAURI_CONF);
    
    let content = fs::read_to_string(&tauri_conf_path)
        .map_err(|e| SyncError::FileRead(TAURI_CONF, e))?;
    
    let mut conf: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| SyncError::JsonParse(TAURI_CONF, e))?;

    let mut changes = Vec::new();

    // 更新 productName
    if !matches!(conf["productName"].as_str(), Some(name) if name == info.capitalized_name) {
        conf["productName"] = serde_json::Value::String(info.capitalized_name.clone());
        changes.push(format!("productName: {}", info.capitalized_name));
    }

    // 更新 version
    if !matches!(conf["version"].as_str(), Some(ver) if ver == info.version) {
        conf["version"] = serde_json::Value::String(info.version.clone());
        changes.push(format!("version: {}", info.version));
    }

    if !changes.is_empty() {
        let new_content = serde_json::to_string_pretty(&conf)
            .map_err(|e| SyncError::JsonSerialize(TAURI_CONF, e))?;
        
        fs::write(&tauri_conf_path, new_content)
            .map_err(|e| SyncError::FileWrite(TAURI_CONF, e))?;
    }

    Ok(())
}

/// 首字母大写
#[inline]
fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

/// 自定义错误类型
#[derive(Debug)]
enum SyncError {
    EnvVar(&'static str),
    InvalidPath(&'static str),
    FileRead(&'static str, std::io::Error),
    FileWrite(&'static str, std::io::Error),
    JsonParse(&'static str, serde_json::Error),
    JsonSerialize(&'static str, serde_json::Error),
    TomlParse(&'static str, toml::de::Error),
    TomlSerialize(&'static str, toml::ser::Error),
    MissingField(&'static str, &'static str),
}

impl fmt::Display for SyncError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SyncError::EnvVar(var) => write!(f, "无法获取环境变量: {}", var),
            SyncError::InvalidPath(msg) => write!(f, "路径错误: {}", msg),
            SyncError::FileRead(file, err) => write!(f, "读取文件 {} 失败: {}", file, err),
            SyncError::FileWrite(file, err) => write!(f, "写入文件 {} 失败: {}", file, err),
            SyncError::JsonParse(file, err) => write!(f, "解析 {} 失败: {}", file, err),
            SyncError::JsonSerialize(file, err) => write!(f, "序列化 {} 失败: {}", file, err),
            SyncError::TomlParse(file, err) => write!(f, "解析 {} 失败: {}", file, err),
            SyncError::TomlSerialize(file, err) => write!(f, "序列化 {} 失败: {}", file, err),
            SyncError::MissingField(field, file) => write!(f, "{} 中缺少字段: {}", file, field),
        }
    }
}

impl std::error::Error for SyncError {}
