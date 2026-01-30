use crate::dns::q_type::DNSRecordType;
use crate::dns::records::DNSRecord;

use std::time::Instant;
use crate::dns::q_class::DNSClass;

// TODO: light LRU system
// Should make a real LRU system for v2 (handle millions of entries)
// Should link them in a HashMap (max_entries: 150_000)
#[allow(unused)]
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub(crate) struct CacheKey {
    pub name: String,
    pub rtype: DNSRecordType,
    pub rclass: DNSClass,
}

#[allow(unused)]
#[derive(Clone, Debug)]
pub(crate) struct CacheEntry {
    pub records: Vec<DNSRecord>,
    pub expires_at: Instant,
    pub last_access: Instant,
}