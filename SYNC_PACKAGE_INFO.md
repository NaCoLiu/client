# 包信息同步说明

## 功能概述

自动将 `package.json` 的信息同步到 `Cargo.toml`，确保前后端版本信息一致。

## 同步内容

从 `package.json` → `Cargo.toml`：

- **name** (名称，首字母自动大写)
- **version** (版本号)

## 工作原理

### 自动同步 (推荐)

每次编译 Rust 代码时，`build.rs` 会自动执行同步：

```bash
# 开发模式
pnpm tauri dev

# 构建模式
pnpm tauri build
```

### 同步流程

1. **编译前检查**: `build.rs` 在编译前运行
2. **读取 package.json**: 提取 `name` 和 `version`
3. **对比 Cargo.toml**: 检查是否需要更新
4. **自动更新**: 如果信息不一致，自动更新 `Cargo.toml`
5. **继续编译**: 使用最新的包信息

### 示例

**package.json**:
```json
{
  "name": "client",
  "version": "0.0.1"
}
```

**自动同步后的 Cargo.toml**:
```toml
[package]
name = "Client"  # 首字母大写
version = "0.0.1"
```

## 构建输出

同步时会在终端显示：

```
warning: 更新名称: Client
warning: 更新版本: 0.0.1
warning: ✓ 已同步 package.json 到 Cargo.toml
```

## 注意事项

1. **名称转换**: `package.json` 中的 `name` 会转换为首字母大写形式
   - `client` → `Client`
   - `my-app` → `My-app`

2. **版本号**: 直接复制，格式应遵循语义化版本规范
   - 推荐格式: `x.y.z` (如 `0.0.1`, `1.2.3`)

3. **作者信息**: 如果需要同步作者，在 `package.json` 中添加：
   ```json
   {
     "author": "Your Name"
   }
   ```

## 手动验证

查看同步后的信息：

```bash
# 查看 Cargo.toml
cat src-tauri/Cargo.toml | grep -A 3 "\[package\]"

# 或在 Windows PowerShell
Get-Content src-tauri/Cargo.toml | Select-String -Pattern "name|version" -Context 0,1
```

## 好处

1. **单一数据源**: 只需在 `package.json` 中维护版本号
2. **自动化**: 无需手动同步，避免遗忘
3. **版本一致**: 确保前后端版本号始终一致
4. **构建时同步**: 每次编译都会检查并更新

## 故障排除

### 同步失败

如果看到警告 "同步 package.json 失败"：

1. 检查 `package.json` 是否存在且格式正确
2. 确保 `name` 和 `version` 字段存在
3. 检查文件权限

### 版本不匹配

如果运行时版本不一致：

1. 清理构建缓存: `pnpm tauri build --clean`
2. 重新构建: `pnpm tauri dev`

## 技术细节

**依赖项**:
- `serde_json`: 解析 JSON
- `toml`: 解析和生成 TOML

**实现文件**:
- `src-tauri/build.rs`: 构建脚本，包含同步逻辑
- `src-tauri/Cargo.toml`: 包含构建依赖配置
