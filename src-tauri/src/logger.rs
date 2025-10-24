use colored::Colorize;

/// 日志类型
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum LogType {
    Success,
    Failure,
    Warning,
}

impl LogType {
    /// 获取日志类型字符串
    #[allow(dead_code)]
    const fn as_str(self) -> &'static str {
        match self {
            Self::Success => "SUCCESS",
            Self::Failure => "FAILURE",
            Self::Warning => "WARNING",
        }
    }

    /// 获取带颜色的日志类型字符串
    #[allow(dead_code)]
    fn colored(self) -> colored::ColoredString {
        match self {
            Self::Success => self.as_str().green(),
            Self::Failure => self.as_str().red(),
            Self::Warning => self.as_str().yellow(),
        }
    }
}

#[allow(dead_code)]
pub fn console_log(log_type: LogType, message: &str) {
    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
    println!("[{}] [{}] {}", timestamp, log_type.colored(), message);
}
