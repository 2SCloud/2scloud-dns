//! Configuration types for scloud-dns
//!
//! This file contains Serde (Deserialize/Serialize) structs that map to the
//! JSON configuration you provided. It includes helpers to load the config
//! from a file and a light `validate()` method placeholder you can extend.

use crate::exceptions::SCloudException;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

/// Top-level configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub server: ServerConfig,

    #[serde(default)]
    pub workers: WorkersConfig,

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
            .with_context(|| format!("reading config file {}", path.display()))
            .map_err(|_| SCloudException::SCLOUD_CONFIG_FILE_NOT_FOUND)?;
        let cfg: Config = serde_json::from_str(&s)
            .context("parsing JSON config")
            .map_err(|_| SCloudException::SCLOUD_CONFIG_IMPOSSIBLE_TO_PARSE_JSON)?;
        cfg.validate()?;
        Ok(cfg)
    }

    /// Validation hook
    pub fn validate(&self) -> Result<(), SCloudException> {
        let acl_names: HashSet<&str> = self.acl.iter().map(|a| a.name.as_str()).collect();
        let tsig_names: HashSet<&str> = self.tsig_key.iter().map(|t| t.name.as_str()).collect();
        let _forwarder_names: HashSet<&str> =
            self.forwarder.iter().map(|f| f.name.as_str()).collect();

        let is_acl_ref_valid = |s: &str| -> bool {
            if s.trim().is_empty() {
                return false;
            }
            acl_names.contains(s) || s.contains('/')
        };

        if self.server.bind_port == 0 {
            return Err(SCloudException::SCLOUD_CONFIG_INVALID_SERVER_PORT);
        }
        if self.server.max_udp_payload == 0 || self.server.max_udp_payload > 65535 {
            return Err(SCloudException::SCLOUD_CONFIG_INVALID_MAX_UDP_PAYLOAD);
        }
        if self.tuning.max_label_length == 0 || self.tuning.max_label_length > 63 {
            return Err(SCloudException::SCLOUD_CONFIG_INVALID_DNS_LIMITS);
        }
        if self.tuning.max_domain_length == 0 || self.tuning.max_domain_length > 253 {
            return Err(SCloudException::SCLOUD_CONFIG_INVALID_DNS_LIMITS);
        }
        if self.limits.max_udp_packet_size == 0 || self.limits.max_udp_packet_size > 65535 {
            return Err(SCloudException::SCLOUD_CONFIG_INVALID_DNS_LIMITS);
        }

        let mut listener_names = HashSet::new();
        for l in &self.listener {
            if l.name.trim().is_empty() {
                return Err(SCloudException::SCLOUD_CONFIG_INVALID_LISTENER);
            }
            if !listener_names.insert(l.name.as_str()) {
                return Err(SCloudException::SCLOUD_CONFIG_DUPLICATE_LISTENER_NAME);
            }
            if l.port == 0 {
                return Err(SCloudException::SCLOUD_CONFIG_INVALID_LISTENER_PORT);
            }
            if l.protocols.is_empty() {
                return Err(SCloudException::SCLOUD_CONFIG_INVALID_LISTENER_PROTOCOLS);
            }
            if !l.acl.trim().is_empty() && !is_acl_ref_valid(&l.acl) {
                return Err(SCloudException::SCLOUD_CONFIG_UNKNOWN_ACL_REFERENCE);
            }

            if l.enable_tls.unwrap_or(false) {
                if l.tls_cert_path.as_deref().unwrap_or("").trim().is_empty() {
                    return Err(SCloudException::SCLOUD_CONFIG_TLS_MISSING_CERT);
                }
                if l.tls_key_path.as_deref().unwrap_or("").trim().is_empty() {
                    return Err(SCloudException::SCLOUD_CONFIG_TLS_MISSING_KEY);
                }
                if !l.protocols.iter().any(|p| matches!(p, Protocol::TCP)) {
                    return Err(SCloudException::SCLOUD_CONFIG_TLS_REQUIRES_TCP);
                }
            }
        }

        if self.doh.enabled {
            if self.doh.paths.is_empty() {
                return Err(SCloudException::SCLOUD_CONFIG_INVALID_DOH);
            }
            if self.doh.terminate_tls {
                if self
                    .doh
                    .tls_cert_path
                    .as_deref()
                    .unwrap_or("")
                    .trim()
                    .is_empty()
                {
                    return Err(SCloudException::SCLOUD_CONFIG_TLS_MISSING_CERT);
                }
                if self
                    .doh
                    .tls_key_path
                    .as_deref()
                    .unwrap_or("")
                    .trim()
                    .is_empty()
                {
                    return Err(SCloudException::SCLOUD_CONFIG_TLS_MISSING_KEY);
                }
            }
        }

        if self.recursion.enabled {
            if self.recursion.allowed_acl.trim().is_empty() {
                return Err(SCloudException::SCLOUD_CONFIG_UNKNOWN_ACL_REFERENCE);
            }
            if !is_acl_ref_valid(&self.recursion.allowed_acl) {
                return Err(SCloudException::SCLOUD_CONFIG_UNKNOWN_ACL_REFERENCE);
            }
        }

        let mut fwd_names = HashSet::new();
        for f in &self.forwarder {
            if f.name.trim().is_empty() {
                return Err(SCloudException::SCLOUD_CONFIG_INVALID_FORWARDER);
            }
            if !fwd_names.insert(f.name.as_str()) {
                return Err(SCloudException::SCLOUD_CONFIG_DUPLICATE_FORWARDER_NAME);
            }
            if f.addresses.is_empty() {
                return Err(SCloudException::SCLOUD_CONFIG_INVALID_FORWARDER);
            }
            for a in &f.addresses {
                if a.parse::<std::net::SocketAddr>().is_err() {
                    return Err(SCloudException::SCLOUD_CONFIG_IMPOSSIBLE_TO_PARSE_ADDR);
                }
            }
        }

        let mut zone_names = HashSet::new();
        for z in &self.zone {
            if z.name.trim().is_empty() {
                return Err(SCloudException::SCLOUD_CONFIG_INVALID_ZONE);
            }
            if !zone_names.insert(z.name.as_str()) {
                return Err(SCloudException::SCLOUD_CONFIG_DUPLICATE_ZONE_NAME);
            }

            match z.kind {
                ZoneType::Master => {
                    let inline = z.inline.unwrap_or(false);
                    if inline {
                        if z.records.is_empty() {
                            return Err(SCloudException::SCLOUD_CONFIG_INVALID_INLINE_ZONE);
                        }
                        let has_soa = z
                            .records
                            .iter()
                            .any(|r| r.r#type.eq_ignore_ascii_case("SOA"));
                        if !has_soa {
                            return Err(SCloudException::SCLOUD_CONFIG_INVALID_INLINE_ZONE);
                        }
                    } else {
                        if z.file.as_deref().unwrap_or("").trim().is_empty() {
                            return Err(SCloudException::SCLOUD_CONFIG_ZONE_MISSING_FILE);
                        }
                    }

                    if let Some(acl) = z.notify_acl.as_deref()
                        && !acl.trim().is_empty()
                        && !is_acl_ref_valid(acl)
                    {
                        return Err(SCloudException::SCLOUD_CONFIG_UNKNOWN_ACL_REFERENCE);
                    }
                    if let Some(acl) = z.allow_transfer_acl.as_deref()
                        && !acl.trim().is_empty()
                        && !is_acl_ref_valid(acl)
                    {
                        return Err(SCloudException::SCLOUD_CONFIG_UNKNOWN_ACL_REFERENCE);
                    }

                    if let Some(k) = z.axfr_tsig_key.as_deref()
                        && !k.trim().is_empty()
                        && !tsig_names.contains(k)
                    {
                        return Err(SCloudException::SCLOUD_CONFIG_UNKNOWN_TSIG_KEY);
                    }
                }
                ZoneType::Slave => {
                    if z.masters.is_empty() {
                        return Err(SCloudException::SCLOUD_CONFIG_SLAVE_MISSING_MASTERS);
                    }
                    for m in &z.masters {
                        if m.parse::<std::net::SocketAddr>().is_err() {
                            return Err(SCloudException::SCLOUD_CONFIG_IMPOSSIBLE_TO_PARSE_ADDR);
                        }
                    }
                    if z.file.as_deref().unwrap_or("").trim().is_empty() {
                        return Err(SCloudException::SCLOUD_CONFIG_ZONE_MISSING_FILE);
                    }
                }
                ZoneType::Forward => {
                    if z.forwarders.is_empty() {
                        return Err(SCloudException::SCLOUD_CONFIG_FORWARD_ZONE_MISSING_FORWARDERS);
                    }
                    for f in &z.forwarders {
                        if f.parse::<std::net::SocketAddr>().is_err() {
                            return Err(SCloudException::SCLOUD_CONFIG_IMPOSSIBLE_TO_PARSE_ADDR);
                        }
                    }
                }
                ZoneType::Stub => {
                    // TODO: not defined JSON yet, strict checks later when I will implement it.
                }
            }

            for r in &z.records {
                if r.r#type.eq_ignore_ascii_case("MX") {
                    if r.priority.is_none() {
                        return Err(SCloudException::SCLOUD_CONFIG_MX_MISSING_PRIORITY);
                    }
                } else if r.priority.is_some() {
                    return Err(SCloudException::SCLOUD_CONFIG_PRIORITY_ON_NON_MX);
                }
            }
        }

        let mut view_names = HashSet::new();
        for v in &self.view {
            if v.name.trim().is_empty() {
                return Err(SCloudException::SCLOUD_CONFIG_INVALID_VIEW);
            }
            if !view_names.insert(v.name.as_str()) {
                return Err(SCloudException::SCLOUD_CONFIG_DUPLICATE_VIEW_NAME);
            }
            if v.acl.trim().is_empty() || !is_acl_ref_valid(&v.acl) {
                return Err(SCloudException::SCLOUD_CONFIG_UNKNOWN_ACL_REFERENCE);
            }
            for vz in &v.zones {
                if vz.name.trim().is_empty() || vz.file.trim().is_empty() {
                    return Err(SCloudException::SCLOUD_CONFIG_INVALID_VIEW);
                }
            }
        }

        for d in &self.dynupdate {
            if d.zone.trim().is_empty() {
                return Err(SCloudException::SCLOUD_CONFIG_INVALID_DYNUPDATE);
            }
            if d.acl.trim().is_empty() || !is_acl_ref_valid(&d.acl) {
                return Err(SCloudException::SCLOUD_CONFIG_UNKNOWN_ACL_REFERENCE);
            }
            if let Some(k) = d.tsig_key.as_deref()
                && !k.trim().is_empty()
                && !tsig_names.contains(k)
            {
                return Err(SCloudException::SCLOUD_CONFIG_UNKNOWN_TSIG_KEY);
            }

            if !zone_names.contains(d.zone.as_str()) {
                return Err(SCloudException::SCLOUD_CONFIG_DYNUPDATE_UNKNOWN_ZONE);
            }
        }

        Ok(())
    }

    /// Get the address of a specific forwarder by index value
    #[allow(unused)]
    pub(crate) fn try_get_forwarder_addr_by_index(
        &self,
        forwarder_index: usize,
        address_index: usize,
    ) -> Result<std::net::SocketAddr, SCloudException> {
        let addr = self
            .forwarder
            .get(forwarder_index)
            .ok_or(SCloudException::SCLOUD_CONFIG_MISSING_FORWARDER)?
            .addresses
            .get(address_index)
            .ok_or(SCloudException::SCLOUD_CONFIG_MISSING_ADDRESS)?
            .parse()
            .map_err(|_| SCloudException::SCLOUD_CONFIG_IMPOSSIBLE_TO_PARSE_ADDR)?;

        Ok(addr)
    }

    // TODO: add a loop to test the next address for each retry
    pub(crate) fn try_get_forwarder_addr_by_name(
        &self,
        forwarder_name: &str,
    ) -> Result<std::net::SocketAddr, SCloudException> {
        let forwarder = self
            .forwarder
            .iter()
            .find(|f| f.name == forwarder_name)
            .ok_or(SCloudException::SCLOUD_CONFIG_MISSING_FORWARDER)?;

        for addr_str in &forwarder.addresses {
            if let Ok(addr) = addr_str.parse::<std::net::SocketAddr>() {
                return Ok(addr);
            }
        }

        Err(SCloudException::SCLOUD_CONFIG_IMPOSSIBLE_TO_PARSE_ADDR)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub name: String,
    pub version: String,
    pub environment: String,
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
            version: "none".to_string(),
            environment: "production".to_string(),
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkersConfig {
    /// Number of UDP listener workers. Each binds the same port with
    /// SO_REUSEPORT so the kernel load-balances datagrams across them.
    #[serde(default = "default_listener_workers")]
    pub listener: u16,
    pub tcp_acceptor: u16,
    #[serde(default)]
    pub doh_acceptor: u16,
    pub decoder: u16,
    pub query_dispatcher: u16,
    pub cache_lookup: u16,
    pub zone_manager: u16,
    pub resolver: u16,
    pub cache_writer: u16,
    pub encoder: u16,
    pub sender: u16,
    pub cache_janitor: u16,
    pub metrics: u16,
}

fn default_listener_workers() -> u16 {
    4
}

impl Default for WorkersConfig {
    fn default() -> Self {
        WorkersConfig {
            listener: 4,
            tcp_acceptor: 1,
            doh_acceptor: 1,
            decoder: 5,
            query_dispatcher: 3,
            cache_lookup: 3,
            zone_manager: 1,
            resolver: 5,
            cache_writer: 1,
            encoder: 5,
            sender: 5,
            cache_janitor: 1,
            metrics: 2,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: LogLevel,
    pub format: LogFormat,
    pub file: String,
    pub dyn_ui: bool,
    pub rotate: bool,
    pub live_print: bool,
    pub max_size_mb: u64,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        LoggingConfig {
            level: LogLevel::INFO,
            format: LogFormat::TEXT,
            file: "/var/log/scloud-dns/scloud-dns.log".to_string(),
            dyn_ui: false,
            rotate: true,
            live_print: false,
            max_size_mb: 200,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    TRACE = 0,
    DEBUG = 1,
    INFO = 2,
    WARN = 3,
    ERROR = 4,
    FATAL = 5,
}

impl LogLevel {
    pub fn parse(s: &str) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "trace" => Self::TRACE,
            "debug" => Self::DEBUG,
            "info" => Self::INFO,
            "warn" | "warning" => Self::WARN,
            "error" => Self::ERROR,
            "fatal" => Self::FATAL,
            _ => Self::WARN,
        }
    }

    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::TRACE => "trace",
            Self::DEBUG => "debug",
            Self::INFO => "info",
            Self::WARN => "warn",
            Self::ERROR => "error",
            Self::FATAL => "fatal",
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LogFormat {
    JSON,
    TEXT,
}

impl LogFormat {
    pub fn parse(s: &str) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "json" => Self::JSON,
            _ => Self::TEXT,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AclEntry {
    pub name: String,
    pub networks: Vec<String>, // CIDRs or single IPs; parse later with ipnet or similar
}

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
    TCP,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DohConfig {
    pub enabled: bool,
    pub bind: String,
    #[serde(default)]
    pub terminate_tls: bool,
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
            bind: "0.0.0.0:8053".to_string(),
            terminate_tls: false,
            tls_cert_path: None,
            tls_key_path: None,
            paths: vec!["/dns-query".to_string()],
            allowed_origins: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForwarderConfig {
    pub name: String,
    pub addresses: Vec<String>,
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
            enabled: false,
            allowed_acl: "internal".to_string(),
            max_recursive_queries: 50,
            recursion_timeout_ms: 5000,
            retry_interval_ms: 200,
        }
    }
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TsigKey {
    pub name: String,
    pub algorithm: String,
    pub secret: String, // TODO: base64 encoded - do not keep in plaintext in production
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PolicyConfig {
    #[serde(default)]
    pub deny_domains: Vec<String>,
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
