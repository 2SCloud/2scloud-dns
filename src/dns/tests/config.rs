use tempfile::NamedTempFile;
use serde_json;
use std::io::Write;
use crate::config::{AxfrConfig, CacheConfig, Config, DnssecConfig, DohConfig, ForwardPolicy, ForwarderConfig, LimitsConfig, ListenerConfig, Protocol, RateLimitConfig, RecursionConfig, ServerConfig, ZoneConfig, ZoneType};

#[test]
fn test_default_config() {
    let cfg = Config::default();

    assert_eq!(cfg.server.name, "scloud-dns");
    assert!(cfg.acl.is_empty());
    assert!(cfg.listener.is_empty());
    assert!(cfg.zone.is_empty());
    assert!(cfg.tsig_key.is_empty());
}

#[test]
fn test_serialize_deserialize() {
    let cfg = Config::default();
    let json = serde_json::to_string(&cfg).expect("Failed to serialize config");
    let cfg2: Config = serde_json::from_str(&json).expect("Failed to deserialize config");

    assert_eq!(cfg2.server.name, cfg.server.name);
    assert_eq!(cfg2.logging.level, cfg.logging.level);
    assert_eq!(cfg2.metrics.enabled, cfg.metrics.enabled);
}

#[test]
fn test_load_from_file() {
    let cfg = Config::default();

    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    let json = serde_json::to_string(&cfg).expect("Failed to serialize config");
    write!(temp_file, "{}", json).expect("Failed to write to temp file");

    let loaded_cfg = Config::from_file(temp_file.path()).expect("Failed to load config from file");
    assert_eq!(loaded_cfg.server.name, cfg.server.name);
    assert_eq!(loaded_cfg.logging.level, cfg.logging.level);
}

#[test]
fn test_server_config_defaults() {
    let server = ServerConfig::default();
    assert_eq!(server.name, "scloud-dns");
    assert_eq!(server.environment, "production");
    assert_eq!(server.bind_port, 53);
    assert!(server.enable_tcp);
    assert!(server.enable_edns);
}

#[test]
fn test_listener_config_defaults() {
    let listener = ListenerConfig::default();
    assert_eq!(listener.address, "0.0.0.0");
    assert_eq!(listener.port, 53);
    assert_eq!(listener.protocols.len(), 1);
    assert!(matches!(listener.protocols[0], Protocol::UDP));
}

#[test]
fn test_zone_config_defaults() {
    let zone = ZoneConfig::default();
    assert_eq!(zone.kind, ZoneType::Master);
    assert_eq!(zone.records.len(), 0);
    assert_eq!(zone.masters.len(), 0);
    assert_eq!(zone.inline, Some(false));
}

#[test]
fn test_forwarder_config_defaults() {
    let forwarder = ForwarderConfig::default();
    assert_eq!(forwarder.policy, ForwardPolicy::First);
    assert_eq!(forwarder.timeout_ms, 1500);
    assert!(forwarder.use_tcp_on_retry.unwrap());
}

#[test]
fn test_doh_config_defaults() {
    let doh = DohConfig::default();
    assert_eq!(doh.enabled, false);
    assert_eq!(doh.bind, "0.0.0.0:443");
    assert_eq!(doh.paths, vec!["/dns-query"]);
}

#[test]
fn test_cache_and_recursion_defaults() {
    let cache = CacheConfig::default();
    let recursion = RecursionConfig::default();
    assert!(cache.enabled);
    assert!(recursion.enabled);
    assert_eq!(recursion.max_recursive_queries, 50);
}

#[test]
fn test_rate_limit_defaults() {
    let ratelimit = RateLimitConfig::default();
    assert!(ratelimit.enabled);
    assert_eq!(ratelimit.rrl.window_seconds, 5);
}

#[test]
fn test_axfr_and_dnssec_defaults() {
    let axfr = AxfrConfig::default();
    let dnssec = DnssecConfig::default();
    assert!(axfr.enabled);
    assert!(!dnssec.enabled);
    assert_eq!(dnssec.default_algo, "RSASHA256");
}

#[test]
fn test_limits_defaults() {
    let limits = LimitsConfig::default();
    assert_eq!(limits.max_udp_packet_size, 4096);
    assert_eq!(limits.max_queries_per_minute_per_ip, 1000);
}

#[test]
fn test_validate_stub() {
    let cfg = Config::default();
    assert!(cfg.validate().is_ok());
}
