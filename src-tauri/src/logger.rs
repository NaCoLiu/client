use colored::*;

#[derive(Debug)]
#[allow(dead_code)]
pub enum LogType {
    SUCCESS,
    FAILURE,
    WARNING,
}
#[allow(dead_code)]
impl LogType {
    fn as_str(&self) -> &str {
        match self {
            LogType::SUCCESS => "SUCCESS",
            LogType::FAILURE => "FAILURE",
            LogType::WARNING => "WARNING",
        }
    }

    fn colored_output(&self) -> ColoredString {
        match self {
            LogType::SUCCESS => self.as_str().green(),
            LogType::FAILURE => self.as_str().red(),
            LogType::WARNING => self.as_str().yellow(),
        }
    }
}
#[allow(dead_code)]
pub fn console_log(log_type: LogType, message: &str) {
    let now = chrono::Local::now();
    let timestamp = now.format("%Y-%m-%d %H:%M:%S");

    println!(
        "[{}] [{}] {}",
        timestamp,
        log_type.colored_output(),
        message
    );
}
