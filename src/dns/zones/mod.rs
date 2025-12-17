pub(crate) mod zone_parser;

use crate::dns::records::DNSRecord;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Zone {
    pub origin: Option<String>,
    pub name: String,
    pub ttl: u32,
    pub soa: Option<DNSRecord>,
    pub records: HashMap<String, Vec<DNSRecord>>,
}
