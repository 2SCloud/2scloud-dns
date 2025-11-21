use std::net::IpAddr;
use std::time::Instant;
use crate::dns::records::DNSRecordType;

pub(crate) struct DNSCacheRecord{
    record_type: DNSRecordType,
    ip_addr: IpAddr,
    domain_name: String,
    last_request: usize,
    ttl: usize,
}