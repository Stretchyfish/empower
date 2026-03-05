use std::collections::VecDeque;

use chrono::{DateTime, Local};

#[derive(Clone)]
pub struct LogBuffer
{
    pub max_entries: usize,
    pub entries: VecDeque<LogEntry>,
}

impl LogBuffer
{
    pub fn new( max_entries: usize ) -> Self
    {
        Self { max_entries, entries: VecDeque::new() }
    }

    pub fn add_log_info(&mut self, text: String)
    {
        let log_entry = LogEntry
        {
            text,
            kind: LogKind::Info,
            timestamp: Local::now(),
        };

        self.add_log_entry(log_entry);
    }

    pub fn add_log_entry(&mut self, log_entry: LogEntry)
    {
        if self.entries.len() > self.max_entries
        {
            self.entries.pop_front();
        }

        self.entries.push_back(log_entry);
    }
}

#[derive(Clone)]
pub struct LogEntry
{
    pub text: String,
    pub kind: LogKind,
    pub timestamp: DateTime<Local>,
}

#[derive(Clone)]
pub enum LogKind
{
    Info,
    Warning,
    Error,
}
