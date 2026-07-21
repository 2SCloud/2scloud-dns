use crate::config::{LogFormat, LogLevel, LoggingConfig};
use crate::exceptions::SCloudException;
use crate::utils::time::{format_system_time, now_epoch_ms};

use once_cell::sync::OnceCell;
use serde_json::json;

use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use std::time::SystemTime;

use tokio::sync::mpsc;

struct Logger {
    cfg: LoggingConfig,
    file: std::fs::File,
}

pub struct OtelLog {
    pub target: String,
    pub severity: &'static str,
    pub message: String,
    pub timestamp_unix_nano: String,
}

pub static LOG_SENDER: OnceCell<mpsc::Sender<OtelLog>> = OnceCell::new();
static LOGGER: OnceLock<Mutex<Logger>> = OnceLock::new();

pub fn build_otlp_payload(logs: &[OtelLog]) -> serde_json::Value {
    json!({
        "resourceLogs": [{
            "resource": {
                "attributes": [
                    { "key": "service.name",        "value": { "stringValue": "scloud-dns" } },
                    { "key": "service.instance.id", "value": { "stringValue": "scloud-dns-01" } }
                ]
            },
            "scopeLogs": [{
                "scope": { "name": "scloud.logger", "version": "0.2.2" },
                "logRecords": logs.iter().map(|log| {
                    json!({
                        "timeUnixNano": log.timestamp_unix_nano,
                        "severityText": log.severity,
                        "body": { "stringValue": log.message },
                        "attributes": [
                            { "key": "target", "value": { "stringValue": log.target } }
                        ]
                    })
                }).collect::<Vec<_>>()
            }]
        }]
    })
}
/// Initialize global logger.
/// Call once at startup.
pub fn init(cfg: LoggingConfig) -> Result<(), SCloudException> {
    let path = Path::new(&cfg.file);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            eprintln!("failed to create log dir {:?}: {}", parent, e);
            SCloudException::SCLOUD_LOGGING_PATH_CREATION_FAILED
        })?;
    }

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|_| SCloudException::SCLOUD_LOGGING_FILE_CREATION_OR_OPENING_FAILED)?;

    let logger = Logger { cfg, file };
    let _ = LOGGER.set(Mutex::new(logger));
    Ok(())
}

/// Write one log line (internal).
/// Safe to call from any thread.
pub fn log(level: LogLevel, target: &str, msg: &str) {
    let Some(lock) = LOGGER.get() else {
        return;
    };

    let mut g = lock.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    if level < g.cfg.level {
        return;
    }

    if g.cfg.live_print {
        let now = SystemTime::now();
        println!(
            "[{}][{:?}][{}] - {}",
            format_system_time(now),
            level,
            target,
            msg
        );
    }

    if g.cfg.rotate {
        let max_bytes = g.cfg.max_size_mb.saturating_mul(1024 * 1024);
        if max_bytes > 0
            && let Ok(meta) = g.file.metadata()
            && meta.len() >= max_bytes
        {
            let _ = rotate_file(&mut g);
        }
    }

    let line = match g.cfg.format {
        LogFormat::JSON => format_json_line(level, target, msg),
        LogFormat::TEXT => format_text_line(level, target, msg),
    };

    let _ = g.file.write_all(line.as_bytes());
    let _ = g.file.write_all(b"\n");
    let _ = g.file.flush();
}

fn rotate_file(logger: &mut Logger) -> io::Result<()> {
    let path = Path::new(&logger.cfg.file);

    let epoch_ms = now_epoch_ms();
    let rotated = rotated_name(path, epoch_ms);

    let _ = logger.file.flush();
    let _ = fs::rename(path, &rotated);

    logger.file = OpenOptions::new().create(true).append(true).open(path)?;
    Ok(())
}

fn rotated_name(path: &Path, epoch_ms: u128) -> PathBuf {
    let mut p = path.as_os_str().to_owned();
    let suffix = format!(".{}", epoch_ms);
    p.push(suffix);
    PathBuf::from(p)
}

fn format_text_line(level: LogLevel, target: &str, msg: &str) -> String {
    format!(
        "[{}][{}][{}] {}",
        format_system_time(SystemTime::now()),
        level.as_str(),
        target,
        msg
    )
}

fn format_json_line(level: LogLevel, target: &str, msg: &str) -> String {
    let msg_esc = json_escape(msg);
    let target_esc = json_escape(target);
    format!(
        r#"{{"ts":{},"level":"{}","target":"{}","msg":"{}"}}"#,
        now_epoch_ms(),
        level.as_str(),
        target_esc,
        msg_esc
    )
}

fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 8);
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c.is_control() => {
                use std::fmt::Write as _;
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            _ => out.push(c),
        }
    }
    out
}

#[macro_export]
macro_rules! __log_internal {
    ($lvl:expr, $otel_lvl:expr, $($arg:tt)*) => {{
        let target = concat!(module_path!(), ":", line!());

        if $otel_lvl != "TRACE" || $otel_lvl != "DEBUG" {
            if $otel_lvl == "STRACE" {
                if let Some(sender) = $crate::utils::logging::LOG_SENDER.get() {
                    let _ = sender.try_send($crate::utils::logging::OtelLog {
                        target: target.to_string(),
                        severity: "TRACE",
                        message: format!($($arg)*),
                        timestamp_unix_nano: $crate::utils::time::now_unix_nano(),
                    });
                }
            } else if $otel_lvl == "SDEBUG" {
                if let Some(sender) = $crate::utils::logging::LOG_SENDER.get() {
                    let _ = sender.try_send($crate::utils::logging::OtelLog {
                        target: target.to_string(),
                        severity: "DEBUG",
                        message: format!($($arg)*),
                        timestamp_unix_nano: $crate::utils::time::now_unix_nano(),
                    });
                }
            }
            else {
                if let Some(sender) = $crate::utils::logging::LOG_SENDER.get() {
                    let _ = sender.try_send($crate::utils::logging::OtelLog {
                        target: target.to_string(),
                        severity: $otel_lvl,
                        message: format!($($arg)*),
                        timestamp_unix_nano: $crate::utils::time::now_unix_nano(),
                    });
                }
            }
        }

        $crate::utils::logging::log(
            $lvl,
            target,
            &format!($($arg)*),
        );
    }};
}

#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)*) => {
        $crate::__log_internal!($crate::config::LogLevel::TRACE, "TRACE", $($arg)*);
    };
}

#[macro_export]
macro_rules! log_strace {
    ($($arg:tt)*) => {
        $crate::__log_internal!($crate::config::LogLevel::TRACE, "STRACE", $($arg)*);
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        $crate::__log_internal!($crate::config::LogLevel::DEBUG, "DEBUG", $($arg)*);
    };
}

#[macro_export]
macro_rules! log_sdebug {
    ($($arg:tt)*) => {
        $crate::__log_internal!($crate::config::LogLevel::DEBUG, "SDEBUG", $($arg)*);
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        $crate::__log_internal!($crate::config::LogLevel::INFO, "INFO", $($arg)*);
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        $crate::__log_internal!($crate::config::LogLevel::WARN, "WARN", $($arg)*);
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        $crate::__log_internal!($crate::config::LogLevel::ERROR, "ERROR", $($arg)*);
    };
}

#[macro_export]
macro_rules! log_fatal {
    ($($arg:tt)*) => {
        $crate::__log_internal!($crate::config::LogLevel::FATAL, "FATAL", $($arg)*);
    };
}
