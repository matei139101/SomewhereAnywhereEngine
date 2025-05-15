#[derive(Debug, Eq, PartialEq, PartialOrd)]
pub enum LogLevel {
    Low = 1,
    Medium = 2,
    High = 3,
    Dev = 4,
}

pub struct Logger {
}

impl Logger {
    pub fn log(level: LogLevel, culprit: &str, message: &str) {
        if level <= LogLevel::Dev {
            println!("{}: [{:?}] ({}) {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"), level, culprit, message);
        }
    }
    
}