use crate::dns::q_class::DNSClass;
use crate::dns::q_type::DNSRecordType;

/// A generic DNS resource record representation.
///
/// `DNSRecord` is a high-level abstraction used to represent DNS records
/// independently of their wire format. It is typically produced after
/// parsing DNS packets and can be used by resolvers, caches, or
/// application-level logic.
///
/// Supported record types include:
/// - A / AAAA
/// - NS
/// - MX
/// - SRV
/// - CAA
/// - NAPTR
///
/// Optional fields are populated depending on the record type.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct DNSRecord {
    pub name: String,
    pub rtype: DNSRecordType,
    pub rclass: DNSClass,
    pub ttl: u32,
    pub value: String,
    pub priority: Option<u16>,       // MX, SRV
    pub weight: Option<u16>,         // SRV
    pub port: Option<u16>,           // SRV
    pub flags: Option<u8>,           // NAPTR, CAA
    pub tag: Option<String>,         // CAA
    pub regex: Option<String>,       // NAPTR
    pub replacement: Option<String>, // NAPTR
    pub order: Option<u16>,          // NAPTR
    pub preference: Option<u16>,     // NAPTR
}
