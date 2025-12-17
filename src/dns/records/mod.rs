use crate::dns::q_class::DNSClass;
use crate::dns::q_type::DNSRecordType;

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
