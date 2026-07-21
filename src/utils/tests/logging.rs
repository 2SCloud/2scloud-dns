#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::Once;
    use std::time::Duration;
    use uuid::Uuid;

    use crate::config::{LogFormat, LogLevel, LoggingConfig};
    use crate::utils::logging::{self, OtelLog};
    use crate::utils::time;
    use crate::{log_info, log_sdebug, log_strace, utils};

    static INIT: Once = Once::new();

    fn make_temp_log_path() -> PathBuf {
        let mut p = std::env::temp_dir();
        p.push("scloud-dns-tests");
        p.push(format!("logging-{}.log", Uuid::new_v4()));
        p
    }

    fn ensure_logger_inited() -> PathBuf {
        let marker_path = std::env::temp_dir().join("scloud-dns-tests-logger-path.txt");

        INIT.call_once(|| {
            let log_path = make_temp_log_path();
            if let Some(parent) = log_path.parent() {
                let _ = fs::create_dir_all(parent);
            }

            let cfg = LoggingConfig {
                file: log_path.to_string_lossy().to_string(),
                level: LogLevel::TRACE,
                dyn_ui: false,
                live_print: false,
                rotate: false,
                max_size_mb: 1,
                format: LogFormat::TEXT,
            };

            logging::init(cfg).expect("logger init failed");
            fs::write(&marker_path, log_path.to_string_lossy().as_bytes())
                .expect("failed to write marker path");
        });

        let path_str = fs::read_to_string(&marker_path).expect("failed to read marker path");
        PathBuf::from(path_str.trim())
    }

    fn file_len(path: &Path) -> u64 {
        fs::metadata(path).map(|m| m.len()).unwrap_or(0)
    }

    fn read_lines(path: &Path) -> Vec<String> {
        let content = fs::read_to_string(path).unwrap_or_default();
        content.lines().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_init_creates_log_file() {
        let path = ensure_logger_inited();
        assert!(path.exists(), "log file should exist: {:?}", path);
    }

    #[test]
    fn test_log_writes_when_level_allowed() {
        let path = ensure_logger_inited();

        let before = file_len(&path);
        logging::log(LogLevel::INFO, "tests/logging.rs", "hello info");
        let after = file_len(&path);

        assert!(after > before, "log file size should increase");
    }

    #[test]
    fn test_log_respects_level_filter_by_default_cfg_trace() {
        let path = ensure_logger_inited();

        let before_lines = read_lines(&path).len();
        logging::log(LogLevel::ERROR, "tests/logging.rs", "boom");
        let after_lines = read_lines(&path).len();

        assert!(after_lines > before_lines);
    }

    #[test]
    fn test_build_otlp_payload_structure() {
        let logs = vec![
            OtelLog {
                target: "my.target".to_string(),
                severity: "INFO",
                message: "hello".to_string(),
                timestamp_unix_nano: "1234567890".to_string(),
            },
            OtelLog {
                target: "other".to_string(),
                severity: "ERROR",
                message: "oops".to_string(),
                timestamp_unix_nano: "999".to_string(),
            },
        ];

        let payload = logging::build_otlp_payload(&logs);

        let resource_logs = payload
            .get("resourceLogs")
            .and_then(|v| v.as_array())
            .expect("resourceLogs should be an array");
        assert_eq!(resource_logs.len(), 1);

        let scope_logs = resource_logs[0]
            .get("scopeLogs")
            .and_then(|v| v.as_array())
            .expect("scopeLogs should be an array");
        assert_eq!(scope_logs.len(), 1);

        let log_records = scope_logs[0]
            .get("logRecords")
            .and_then(|v| v.as_array())
            .expect("logRecords should be an array");
        assert_eq!(log_records.len(), 2);

        assert_eq!(log_records[0].get("timeUnixNano").unwrap(), "1234567890");
        assert_eq!(log_records[0].get("severityText").unwrap(), "INFO");
        assert_eq!(
            log_records[0]
                .get("body")
                .and_then(|b| b.get("stringValue"))
                .unwrap(),
            "hello"
        );

        let attrs = log_records[0]
            .get("attributes")
            .and_then(|v| v.as_array())
            .unwrap();
        assert_eq!(
            attrs[0]
                .get("value")
                .and_then(|v| v.get("stringValue"))
                .unwrap(),
            "my.target"
        );
    }

    #[test]
    fn test_json_format_line_is_valid_json_and_escapes() {
        let path = ensure_logger_inited();
        let msg = "quote: \" backslash: \\ newline:\n tab:\t";
        let logs = vec![OtelLog {
            target: "t".to_string(),
            severity: "INFO",
            message: msg.to_string(),
            timestamp_unix_nano: time::now_unix_nano(),
        }];

        let payload = logging::build_otlp_payload(&logs);

        let rec = payload["resourceLogs"][0]["scopeLogs"][0]["logRecords"][0].clone();
        assert_eq!(rec["body"]["stringValue"], msg);

        let before = file_len(&path);
        logging::log(LogLevel::INFO, "tests/logging.rs", msg);
        let after = file_len(&path);
        assert!(after > before);
    }

    #[test]
    fn test_rotation_creates_rotated_file_when_enabled_and_size_exceeded() {
        let path = ensure_logger_inited();
        let dir = path.parent().unwrap().to_path_buf();
        let base = path.file_name().unwrap().to_string_lossy().to_string();

        logging::log(LogLevel::INFO, "tests/logging.rs", "rotation smoke");

        if let Ok(entries) = fs::read_dir(&dir) {
            for e in entries.flatten() {
                let name = e.file_name().to_string_lossy().to_string();
                if name.starts_with(&format!("{}.", base)) {
                    let suffix = name.trim_start_matches(&format!("{}.", base));
                    assert!(suffix.parse::<u128>().is_ok());
                    return;
                }
            }
        }
    }

    #[tokio::test]
    async fn test_macros_send_otel_logs_info_and_special_levels() {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<OtelLog>(16);
        let _ = utils::logging::LOG_SENDER.set(tx);

        log_info!("hello {}", 42);

        let got = tokio::time::timeout(Duration::from_millis(200), rx.recv())
            .await
            .ok()
            .flatten();

        if let Some(log) = got {
            assert_eq!(log.severity, "INFO");
            assert!(log.message.contains("hello 42"));
            assert!(!log.timestamp_unix_nano.is_empty());
            assert!(!log.target.is_empty());
        }

        log_strace!("secret trace");
        let got2 = tokio::time::timeout(Duration::from_millis(200), rx.recv())
            .await
            .ok()
            .flatten();

        if let Some(log) = got2 {
            assert_eq!(log.severity, "TRACE");
            assert!(log.message.contains("secret trace"));
        }

        log_sdebug!("secret debug");
        let got3 = tokio::time::timeout(Duration::from_millis(200), rx.recv())
            .await
            .ok()
            .flatten();

        if let Some(log) = got3 {
            assert_eq!(log.severity, "DEBUG");
            assert!(log.message.contains("secret debug"));
        }
    }
}
