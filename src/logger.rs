use chrono::{DateTime, Local};
use color_eyre::eyre::{eyre, Result};
use std::collections::HashSet;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::sync::{Mutex, OnceLock};

static LOG_FILE: OnceLock<Mutex<File>> = OnceLock::new();
static LOGGED_MESSAGES: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();

/// Initialize the logger with the path where log lines will be appended.
/// Creates parent directories if they are missing and truncates nothing.
pub fn init<P: AsRef<Path>>(path: P) -> Result<()> {
    if LOG_FILE.get().is_some() {
        return Ok(());
    }

    let path = path.as_ref();

    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    let file = OpenOptions::new().create(true).append(true).open(path)?;

    LOG_FILE
        .set(Mutex::new(file))
        .map_err(|_| eyre!("logger was already initialized"))?;

    Ok(())
}

/// Write a log line with INFO level.
pub fn info(message: impl AsRef<str>) -> Result<()> {
    log("INFO", message)
}

/// Write a log line with ERROR level.
pub fn error(message: impl AsRef<str>) -> Result<()> {
    log("ERROR", message)
}

/// Write a log line with DEBUG level.
pub fn debug(message: impl AsRef<str>) -> Result<()> {
    log("DEBUG", message)
}

fn log(level: &str, message: impl AsRef<str>) -> Result<()> {
    let file = LOG_FILE
        .get()
        .ok_or_else(|| eyre!("logger is not initialized"))?;

    let mut file = file.lock().expect("logger file mutex poisoned");

    let datetime: DateTime<Local> = Local::now();

    writeln!(
        file,
        "[{:02}] {}: {}",
        datetime.format("%Y-%m-%d %H:%M:%S"),
        level,
        message.as_ref()
    )?;
    file.flush()?;
    Ok(())
}

pub fn log_once<M, F>(message: M, log_fn: F)
where
    M: AsRef<str>,
    F: FnOnce(&str) -> Result<()>,
{
    let set = LOGGED_MESSAGES.get_or_init(|| Mutex::new(HashSet::new()));
    let msg = message.as_ref().to_owned();

    if let Ok(mut guard) = set.lock() {
        if guard.insert(msg.clone()) {
            let _ = log_fn(&msg);
        }
    }
}
