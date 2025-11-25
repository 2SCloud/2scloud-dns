use crate::dns::records::DNSRecordType;
use std::net::IpAddr;
use std::time::Instant;

pub(crate) struct DNSCacheRecord {
    record_type: DNSRecordType,
    ip_addr: IpAddr,
    domain_name: String,
    last_request: usize,
    ttl: usize,
}
