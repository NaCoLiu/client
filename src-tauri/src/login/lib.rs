use super::request::request_connection_server;

pub async fn check_server_connection() -> bool {
    request_connection_server().await
}

pub async fn get_screen_hwid() -> String {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        
        // 使用 PowerShell 获取显示器 PNPDeviceID (Win11 兼容)
        let output = Command::new("powershell")
            .args(&[
                "-NoProfile",
                "-Command",
                "Get-PnpDevice -Class Monitor | Select-Object -First 1 -ExpandProperty InstanceId"
            ])
            .output();
        
        match output {
            Ok(output) if output.status.success() => {
                let raw_hwid = String::from_utf8_lossy(&output.stdout)
                    .trim()
                    .to_string();
                
                // 如果获取到硬件ID，进行MD5加密
                if !raw_hwid.is_empty() {
                    let digest = md5::compute(raw_hwid.as_bytes());
                    format!("{:x}", digest)
                } else {
                    String::new()
                }
            }
            _ => String::new(),
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        String::new()
    }
}