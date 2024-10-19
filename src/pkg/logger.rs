use chrono::{SecondsFormat, Utc};
use log::Record;
use serde::Serialize;
use std::io::Write;

fn clean_string(target: &str, input: &str) -> String {
    if target == "sqlx::query" {
        return input
            .replace("\\\"", "")
            .replace("\\n ", "")
            .replace("\\n", " ")
            .replace("  ", "");
    }

    String::from(input)
}

#[derive(Serialize)]
struct LogEntry {
    timestamp: String,
    level: String,
    target: String,
    message: String,
}

pub fn format_logger<W: Write>(buf: &mut W, record: &Record) -> std::io::Result<()> {
    let level = record.level();
    let message = record.args().to_string();
    let timestamp = Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true);
    // Create an instance of LogEntry
    let log_entry: LogEntry = LogEntry {
        timestamp,
        level: level.to_string(),
        target: record.target().to_string(),
        message: clean_string(record.target(), &message),
    };

    // Serialize the log entry to JSON
    let json_object = serde_json::to_string(&log_entry)?;

    writeln!(buf, "{}", json_object.to_string())
}