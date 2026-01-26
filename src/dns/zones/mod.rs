pub(crate) mod zone_parser;

use crate::dns::records::DNSRecord;
use std::collections::HashMap;

/// Represents a DNS zone.
///
/// A `Zone` contains all resource records associated with a DNS zone,
/// typically loaded from a zone file. It includes metadata such as the
/// origin, default TTL, SOA record, and all other records grouped by name.
///
/// This structure is commonly used by authoritative DNS servers and
/// zone file parsers.
///
/// - `origin`: Optional zone origin (e.g. "example.com.")
/// - `name`: Zone name
/// - `ttl`: Default TTL for records in the zone
/// - `soa`: Start of Authority (SOA) record, if present
/// - `records`: DNS records indexed by owner name
///
/// # Exemple :
/// ```
/// use std::collections::HashMap;
/// use crate::dns::zone::Zone;
/// use crate::dns::records::DNSRecord;
/// use crate::dns::q_type::DNSRecordType;
/// use crate::dns::q_class::DNSClass;
///
/// let soa = DNSRecord {
///     name: "example.com".to_string(),
///     rtype: DNSRecordType::SOA,
///     rclass: DNSClass::IN,
///     ttl: 3600,
///     value: "ns1.example.com hostmaster.example.com".to_string(),
///     priority: None,
///     weight: None,
///     port: None,
///     flags: None,
///     tag: None,
///     regex: None,
///     replacement: None,
///     order: None,
///     preference: None,
/// };
///
/// let mut records = HashMap::new();
/// records.insert(
///     "example.com".to_string(),
///     vec![DNSRecord {
///         name: "example.com".to_string(),
///         rtype: DNSRecordType::A,
///         rclass: DNSClass::IN,
///         ttl: 300,
///         value: "93.184.216.34".to_string(),
///         priority: None,
///         weight: None,
///         port: None,
///         flags: None,
///         tag: None,
///         regex: None,
///         replacement: None,
///         order: None,
///         preference: None,
///     }],
/// );
///
/// let zone = Zone {
///     origin: Some("example.com.".to_string()),
///     name: "example.com".to_string(),
///     ttl: 3600,
///     soa: Some(soa),
///     records,
/// };
///
/// assert_eq!(zone.name, "example.com");
/// ```
#[derive(Debug, PartialEq)]
pub struct Zone {
    pub origin: Option<String>,
    pub name: String,
    pub ttl: u32,
    pub soa: Option<DNSRecord>,
    pub records: HashMap<String, Vec<DNSRecord>>,
}

impl Zone {
    /// Get all DNS records for a given name.
    ///
    /// The name must match the record owner name exactly as stored
    /// in the zone (usually a fully-qualified domain name).
    ///
    /// # Exemple :
    /// ```
    /// use crate::dns::zone::Zone;
    /// use crate::dns::records::DNSRecord;
    ///
    /// let records = zone.get_records("example.com");
    ///
    /// if let Some(records) = records {
    ///     assert!(!records.is_empty());
    /// }
    /// ```
    pub fn get_records(&self, name: &str) -> Option<&Vec<DNSRecord>> {
        self.records.get(name)
    }
}
