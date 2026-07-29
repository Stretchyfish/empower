use std::time::Instant;

pub struct Log
{
    pub text: String,
    pub creation_time: Instant,
    pub level: LogLevel,
}

impl Log
{
    pub fn info(text: &'static str) -> Self
    {
        Self
        {
            text: String::from(text),
            creation_time: Instant::now(),
            level: LogLevel::Info
        }
    }

    pub fn warning(text: &'static str) -> Self
    {
        Self
        {
            text: String::from(text),
            creation_time: Instant::now(),
            level: LogLevel::Warning
        }
    }
}

pub enum LogLevel
{
    Info,
    Warning,
}
