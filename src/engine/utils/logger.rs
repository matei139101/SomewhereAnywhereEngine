#[derive(Debug, Eq, PartialEq, PartialOrd)]
pub enum LogLevel {
    Low = 1,
    Medium = 2,
    High = 3,
}

pub struct Logger {
    level: LogLevel,
}

impl Logger {
    pub fn log(level: LogLevel, culprit: &str, message: &str) {
        if level <= LogLevel::High {
            println!("[{:?}] ({}): {}", level, culprit, message);
        }
    }
    
}