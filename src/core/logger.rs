//! Logging subsystem with log level filtering and persistent output.
//!
//! Mirrors `core/utils/logger.h` and `core/utils/logger.cpp`.

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::SystemTime;

/// Log severity levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Debug,
    Notice,
    Warning,
    Error,
    Critical,
    Time,
}

impl LogLevel {
    /// Returns the prefix string for this log level.
    pub fn as_prefix(&self) -> &'static str {
        match self {
            LogLevel::Debug => "DEBUG: ",
            LogLevel::Notice => "NOTICE: ",
            LogLevel::Warning => "WARNING: ",
            LogLevel::Error => "ERROR: ",
            LogLevel::Critical => "CRITICAL: ",
            LogLevel::Time => "TIME: ",
        }
    }
}

/// Logger that writes structured entries to a persistent file (such as on SD card).
pub struct Logger {
    filename: PathBuf,
    file: Mutex<Option<File>>,
}

impl Logger {
    /// Maximum format length buffer size.
    pub const MAX_FORMAT_LEN: usize = 512;

    /// Creates a new logger targeting the given file.
    ///
    /// Truncates or creates the file upon creation.
    pub fn new<P: AsRef<Path>>(filename: P) -> Self {
        let path = filename.as_ref().to_path_buf();
        // Create or overwrite initially
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&path)
            .ok();

        Self {
            filename: path,
            file: Mutex::new(file),
        }
    }

    fn write_raw(&self, content: &str) {
        let mut guard = self.file.lock().unwrap();
        if guard.is_none() {
            *guard = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&self.filename)
                .ok();
        }
        if let Some(ref mut file) = *guard {
            let _ = file.write_all(content.as_bytes());
            let _ = file.flush();
        }
    }

    /// Logs raw string content without level prefix.
    pub fn log(&self, message: &str) {
        self.write_raw(message);
    }

    /// Logs string content with a specific log level.
    pub fn log_level(&self, level: LogLevel, message: &str) {
        if level == LogLevel::Time {
            let now = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map(|d| d.as_millis())
                .unwrap_or(0);
            self.write_raw(&format!("{}: {}", now, message));
        } else {
            self.write_raw(&format!("{}{}", level.as_prefix(), message));
        }
    }

    /// Logs string content followed by a newline.
    pub fn logln(&self, message: &str) {
        self.write_raw(&format!("{}\n", message));
    }

    /// Logs string content with a log level followed by a newline.
    pub fn logln_level(&self, level: LogLevel, message: &str) {
        if level == LogLevel::Time {
            let now = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map(|d| d.as_millis())
                .unwrap_or(0);
            self.write_raw(&format!("{}: {}\n", now, message));
        } else {
            self.write_raw(&format!("{}{}\n", level.as_prefix(), message));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger() {
        let dir = std::env::temp_dir();
        let path = dir.join("test_log.txt");
        let logger = Logger::new(&path);

        logger.logln_level(LogLevel::Debug, "Testing debug log");
        logger.logln_level(LogLevel::Warning, "Testing warning log");

        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.contains("DEBUG: Testing debug log\n"));
        assert!(content.contains("WARNING: Testing warning log\n"));

        let _ = std::fs::remove_file(path);
    }
}
