//! Configuration types for scloud-dns
//!
//! This file contains Serde (Deserialize/Serialize) structs that map to the
//! JSON configuration you provided. It includes helpers to load the config
//! from a file and a light `validate()` method placeholder you can extend.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use crate::exceptions::SCloudException;

/// Top-level configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub server: ServerConfig,

    #[serde(default)]
    pub logging: LoggingConfig,

    #[serde(default)]
    pub metrics: MetricsConfig,

    #[serde(default)]
    pub admin: AdminConfig,

    #[serde(default)]
    pub acl: Vec<AclEntry>,

    #[serde(default)]
    pub listener: Vec<ListenerConfig>,

    #[serde(default)]
    pub doh: DohConfig,

    #[serde(default)]
    pub forwarder: Vec<ForwarderConfig>,

    #[serde(default)]
    pub root_hints: RootHintsConfig,

    #[serde(default)]
    pub cache: CacheConfig,

    #[serde(default)]
    pub recursion: RecursionConfig,

    #[serde(default)]
    pub ratelimit: RateLimitConfig,

    #[serde(default)]
    pub zone: Vec<ZoneConfig>,

    #[serde(default)]
    pub tsig_key: Vec<TsigKey>,

    #[serde(default)]
    pub axfr: AxfrConfig,

    #[serde(default)]
    pub dnssec: DnssecConfig,

    #[serde(default)]
    pub policy: PolicyConfig,

    #[serde(default)]
    pub amplification_mitigation: AmplificationMitigationConfig,

    #[serde(default)]
    pub tuning: TuningConfig,

    #[serde(default)]
    pub view: Vec<ViewConfig>,

    #[serde(default)]
    pub monitoring: MonitoringConfig,

    #[serde(default)]
    pub dynupdate: Vec<DynUpdateConfig>,

    #[serde(default)]
    pub limits: LimitsConfig,
}

impl Config {
    /// Load config from a JSON file path
    pub fn from_file(path: &Path) -> Result<Self, SCloudException> {
        let s = fs::read_to_string(path)
            .with_context(|| format!("reading config file {}", path.display())).map_err(|_| SCloudException::SCLOUD_CONFIG_FILE_NOT_FOUND);
        let cfg: Config = serde_json::from_str(&s.unwrap()).context("parsing JSON config").map_err(|_| SCloudException::SCLOUD_CONFIG_IMPOSSIBLE_TO_PARSE_JSON)?;
        // cfg.validate()?;
        Ok(cfg)
    }

    /// Basic validation hook — extend with more checks as needed
    pub fn validate(&self) -> Result<()> {
        // Example checks:
        // - ensure no duplicate zone names
        // - ensure TSIG key references exist for zones that reference them
        // - ensure listener port ranges, etc.
        // For now, just return Ok.
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            logging: LoggingConfig::default(),
            metrics: MetricsConfig::default(),
            admin: AdminConfig::default(),
            acl: Vec::new(),
            listener: Vec::new(),
            doh: DohConfig::default(),
            forwarder: Vec::new(),
            root_hints: RootHintsConfig::default(),
            cache: CacheConfig::default(),
            recursion: RecursionConfig::default(),
            ratelimit: RateLimitConfig::default(),
            zone: Vec::new(),
            tsig_key: Vec::new(),
            axfr: AxfrConfig::default(),
            dnssec: DnssecConfig::default(),
            policy: PolicyConfig::default(),
            amplification_mitigation: AmplificationMitigationConfig::default(),
            tuning: TuningConfig::default(),
            view: Vec::new(),
            monitoring: MonitoringConfig::default(),
            dynupdate: Vec::new(),
            limits: LimitsConfig::default(),
        }
    }
}

/* ---------------------------
   Server / runtime settings
   --------------------------- */

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub name: String,
    pub environment: String,
    pub workers: usize,
    pub max_concurrent_requests: usize,
    pub graceful_shutdown_timeout_secs: u64,

    pub default_ttl: u32,
    pub max_udp_payload: usize,
    pub enable_edns: bool,
    pub enable_tcp: bool,
    pub enable_dnssec: bool,

    pub bind_port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            name: "scloud-dns".to_string(),
            environment: "production".to_string(),
            workers: 8,
            max_concurrent_requests: 5000,
            graceful_shutdown_timeout_secs: 15,
            default_ttl: 3600,
            max_udp_payload: 4096,
            enable_edns: true,
            enable_tcp: true,
            enable_dnssec: false,
            bind_port: 53,
        }
    }
}

/* ---------------------------
   Logging / metrics / admin
   --------------------------- */

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
    pub file: String,
    pub rotate: bool,
    pub max_size_mb: u64,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        LoggingConfig {
            level: "info".to_string(),
            format: "json".to_string(),
            file: "/var/log/scloud-dns/scloud-dns.log".to_string(),
            rotate: true,
            max_size_mb: 200,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    pub enabled: bool,
    pub prometheus_bind: String,
    pub enable_health_endpoint: bool,
    pub health_bind: String,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        MetricsConfig {
            enabled: true,
            prometheus_bind: "0.0.0.0:9153".to_string(),
            enable_health_endpoint: true,
            health_bind: "127.0.0.1:8081".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminConfig {
    pub enabled: bool,
    pub bind: String,
    pub auth_token: String,
    pub enable_tls: bool,
}

impl Default for AdminConfig {
    fn default() -> Self {
        AdminConfig {
            enabled: true,
            bind: "127.0.0.1:8053".to_string(),
            auth_token: "replace-with-secure-token".to_string(),
            enable_tls: false,
        }
    }
}

/* ---------------------------
   ACLs
   --------------------------- */

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AclEntry {
    pub name: String,
    pub networks: Vec<String>, // CIDRs or single IPs; parse later with ipnet or similar
}

/* ---------------------------
   Listeners (UDP/TCP/DoT)
   --------------------------- */

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListenerConfig {
    pub name: String,
    pub address: String,
    pub port: u16,
    #[serde(default)]
    pub protocols: Vec<Protocol>,
    #[serde(default)]
    pub recursion_allowed: bool,
    /// ACL name or a raw CIDR/list string
    #[serde(default)]
    pub acl: String,
    #[serde(default)]
    pub workers: Option<usize>,
    #[serde(default)]
    pub enable_tls: Option<bool>,
    #[serde(default)]
    pub tls_cert_path: Option<String>,
    #[serde(default)]
    pub tls_key_path: Option<String>,
}

impl Default for ListenerConfig {
    fn default() -> Self {
        ListenerConfig {
            name: String::new(),
            address: "0.0.0.0".to_string(),
            port: 53,
            protocols: vec![Protocol::UDP],
            recursion_allowed: false,
            acl: "0.0.0.0/0".to_string(),
            workers: None,
            enable_tls: None,
            tls_cert_path: None,
            tls_key_path: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    UDP,
    TCP
}

/* ---------------------------
   DoH
   --------------------------- */

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DohConfig {
    pub enabled: bool,
    pub bind: String,
    #[serde(default)]
    pub tls_cert_path: Option<String>,
    #[serde(default)]
    pub tls_key_path: Option<String>,
    #[serde(default)]
    pub paths: Vec<String>,
    #[serde(default)]
    pub allowed_origins: Vec<String>,
}

impl Default for DohConfig {
    fn default() -> Self {
        DohConfig {
            enabled: false,
            bind: "0.0.0.0:443".to_string(),
            tls_cert_path: None,
            tls_key_path: None,
            paths: vec!["/dns-query".to_string()],
            allowed_origins: Vec::new(),
        }
    }
}

/* ---------------------------
   Forwarders / root hints
   --------------------------- */

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForwarderConfig {
    pub name: String,
    pub addresses: Vec<String>, // host:port
    pub policy: ForwardPolicy,
    pub timeout_ms: u64,
    pub edns: bool,
    pub use_tcp_on_retry: Option<bool>,
}

impl Default for ForwarderConfig {
    fn default() -> Self {
        ForwarderConfig {
            name: String::new(),
            addresses: Vec::new(),
            policy: ForwardPolicy::First,
            timeout_ms: 1500,
            edns: true,
            use_tcp_on_retry: Some(true),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(PartialEq)]
pub enum ForwardPolicy {
    RoundRobin,
    First,
    Random,
}

/* ---------------------------
   Root hints
   --------------------------- */

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootHintsConfig {
    pub file: String,
}

impl Default for RootHintsConfig {
    fn default() -> Self {
        RootHintsConfig {
            file: "/etc/scloud/root.hints".to_string(),
        }
    }
}

/* ---------------------------
   Cache & recursion
   --------------------------- */

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub enabled: bool,
    pub max_entries: usize,
    pub max_ttl_seconds: u64,
    pub negative_ttl_seconds: u64,
    pub eviction_policy: String,
}

impl Default for CacheConfig {
    fn default() -> Self {
        CacheConfig {
            enabled: true,
            max_entries: 200_000,
            max_ttl_seconds: 86_400,
            negative_ttl_seconds: 300,
            eviction_policy: "lru".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecursionConfig {
    pub enabled: bool,
    pub allowed_acl: String,
    pub max_recursive_queries: usize,
    pub recursion_timeout_ms: u64,
    pub retry_interval_ms: u64,
}

impl Default for RecursionConfig {
    fn default() -> Self {
        RecursionConfig {
            enabled: true,
            allowed_acl: "internal".to_string(),
            max_recursive_queries: 50,
            recursion_timeout_ms: 5000,
            retry_interval_ms: 200,
        }
    }
}

/* ---------------------------
   Rate limiting / RRL
   --------------------------- */

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub enabled: bool,
    pub global_qps: u64,
    pub per_ip_qps: u64,
    pub per_subnet_qps: u64,
    pub rrl: RrlConfig,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        RateLimitConfig {
            enabled: true,
            global_qps: 3000,
            per_ip_qps: 100,
            per_subnet_qps: 1000,
            rrl: RrlConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RrlConfig {
    pub enabled: bool,
    pub window_seconds: u64,
    pub slip: u32,
    pub qps_threshold: u64,
}

impl Default for RrlConfig {
    fn default() -> Self {
        RrlConfig {
            enabled: true,
            window_seconds: 5,
            slip: 2,
            qps_threshold: 50,
        }
    }
}

/* ---------------------------
   Zones & Records
   --------------------------- */

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneConfig {
    pub name: String,
    #[serde(rename = "type")]
    pub kind: ZoneType,
    #[serde(default)]
    pub file: Option<String>,
    #[serde(default)]
    pub notify: Option<bool>,
    #[serde(default)]
    pub notify_acl: Option<String>,
    #[serde(default)]
    pub allow_transfer_acl: Option<String>,
    #[serde(default)]
    pub allow_update_acl: Option<String>,
    #[serde(default)]
    pub axfr_tsig_key: Option<String>,

    // Slave-specific
    #[serde(default)]
    pub masters: Vec<String>,

    // Inline zone
    #[serde(default)]
    pub inline: Option<bool>,
    #[serde(default)]
    pub records: Vec<ZoneRecord>,

    // Forward-specific
    #[serde(default)]
    pub forwarders: Vec<String>,
    #[serde(default)]
    pub forward_policy: Option<String>,
}

impl Default for ZoneConfig {
    fn default() -> Self {
        ZoneConfig {
            name: String::new(),
            kind: ZoneType::Master,
            file: None,
            notify: Some(false),
            notify_acl: None,
            allow_transfer_acl: None,
            allow_update_acl: None,
            axfr_tsig_key: None,
            masters: Vec::new(),
            inline: Some(false),
            records: Vec::new(),
            forwarders: Vec::new(),
            forward_policy: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[derive(PartialEq)]
pub enum ZoneType {
    Master,
    Slave,
    Forward,
    Stub,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneRecord {
    pub name: String,
    pub ttl: Option<u32>,
    pub class: Option<String>,
    #[serde(rename = "type")]
    pub r#type: String,
    pub rdata: String,
    #[serde(default)]
    pub priority: Option<u16>,
}

/* ---------------------------
   TSIG / AXFR / DNSSEC
   --------------------------- */

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TsigKey {
    pub name: String,
    pub algorithm: String,
    pub secret: String, // base64 encoded - do not keep in plaintext in production
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AxfrConfig {
    pub enabled: bool,
    pub max_concurrent_transfers: usize,
    pub transfer_timeout_secs: u64,
}

impl Default for AxfrConfig {
    fn default() -> Self {
        AxfrConfig {
            enabled: true,
            max_concurrent_transfers: 4,
            transfer_timeout_secs: 120,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnssecConfig {
    pub enabled: bool,
    pub auto_sign: bool,
    pub default_algo: String,
    pub kasp_file: Option<String>,
}

impl Default for DnssecConfig {
    fn default() -> Self {
        DnssecConfig {
            enabled: false,
            auto_sign: false,
            default_algo: "RSASHA256".to_string(),
            kasp_file: None,
        }
    }
}

/* ---------------------------
   Policy / mitigation / tuning
   --------------------------- */

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyConfig {
    #[serde(default)]
    pub deny_domains: Vec<String>,
}

impl Default for PolicyConfig {
    fn default() -> Self {
        PolicyConfig {
            deny_domains: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmplificationMitigationConfig {
    pub drop_fragments: bool,
    pub max_response_size_udp: usize,
}

impl Default for AmplificationMitigationConfig {
    fn default() -> Self {
        AmplificationMitigationConfig {
            drop_fragments: true,
            max_response_size_udp: 4096,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuningConfig {
    pub socket_recv_buffer_bytes: usize,
    pub socket_send_buffer_bytes: usize,
    pub max_label_length: usize,
    pub max_domain_length: usize,
}

impl Default for TuningConfig {
    fn default() -> Self {
        TuningConfig {
            socket_recv_buffer_bytes: 262_144,
            socket_send_buffer_bytes: 262_144,
            max_label_length: 63,
            max_domain_length: 253,
        }
    }
}

/* ---------------------------
   Views (split-horizon)
   --------------------------- */

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewConfig {
    pub name: String,
    pub acl: String,
    #[serde(default)]
    pub zones: Vec<ViewZone>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewZone {
    pub name: String,
    pub file: String,
}

/* ---------------------------
   Monitoring / dynamic updates / limits
   --------------------------- */

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub enable_query_logging: bool,
    pub query_log_path: String,
    pub log_query_qps: u64,
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        MonitoringConfig {
            enable_query_logging: false,
            query_log_path: "/var/log/scloud-dns/queries.log".to_string(),
            log_query_qps: 1000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynUpdateConfig {
    pub zone: String,
    pub acl: String,
    pub tsig_key: Option<String>,
    pub allow: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitsConfig {
    pub max_udp_packet_size: usize,
    pub max_queries_per_minute_per_ip: u64,
    pub max_tcp_sessions_per_ip: usize,
}

impl Default for LimitsConfig {
    fn default() -> Self {
        LimitsConfig {
            max_udp_packet_size: 4096,
            max_queries_per_minute_per_ip: 1000,
            max_tcp_sessions_per_ip: 8,
        }
    }
}