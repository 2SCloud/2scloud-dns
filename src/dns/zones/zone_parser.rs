use crate::dns::q_class::DNSClass;
use crate::dns::q_type::DNSRecordType;
use crate::dns::records::DNSRecord;
use crate::dns::zones::Zone;
use crate::exceptions::SCloudException;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

/// Parse a DNS zone file and build an in-memory `Zone` structure.
///
/// The zone file must be located in the `zones/` directory and named
/// `<qname>.zone`.
///
/// This parser supports:
/// - `$TTL` directive (default TTL)
/// - `$ORIGIN` directive
/// - SOA record (unique per zone)
/// - Common DNS record types: A, AAAA, NS, MX, TXT, SOA, CNAME, PTR, SRV, CAA, NAPTR
///
/// Records are stored by owner name in a `HashMap<String, Vec<DNSRecord>>`.
/// The SOA record is stored separately in `zone.soa`.
///
/// # Arguments
/// * `qname` - The zone name (used to locate the zone file)
///
/// # Errors
/// Returns `SCloudException` if:
/// - the zone file cannot be found
/// - the file is empty or unreadable
/// - TTL parsing fails
///
/// # Example
/// ```
/// use crate::dns::zones::zone_parser;
///
/// let zone = zone_parser("example.com").expect("Failed to parse zone");
///
/// assert!(zone.soa.is_some());
/// assert!(!zone.records.is_empty());
/// ```
pub fn zone_parser(qname: &str) -> Result<Zone, SCloudException> {
    let filename = format!("zones/{}.zone", qname);
    let file =
        File::open(&filename).map_err(|_| SCloudException::SCLOUD_ZONE_PARSER_FILE_NOT_FOUND)?;

    let mut zone = Zone {
        origin: None,
        name: String::new(),
        ttl: 3600,
        soa: None,
        records: HashMap::new(),
    };

    let mut default_ttl = 3600u32;

    for line in io::BufReader::new(file).lines() {
        let line = line.map_err(|_| SCloudException::SCLOUD_ZONE_PARSER_FILE_EMPTY)?;
        let line = line.trim();

        if line.is_empty() || line.starts_with(';') {
            continue;
        }

        let line = if let Some(idx) = line.find(';') {
            &line[..idx]
        } else {
            line
        }
        .trim();

        if line.starts_with("$TTL") {
            if let Some(ttl_str) = line.split_whitespace().nth(1) {
                default_ttl = ttl_str
                    .parse::<u32>()
                    .map_err(|_| SCloudException::SCLOUD_ZONE_PARSER_FAILED_TO_READ_TTL_FIELD)?;
                zone.ttl = default_ttl;
            }
            continue;
        }

        if line.starts_with("$ORIGIN") {
            if let Some(origin_str) = line.split_whitespace().nth(1) {
                zone.origin = Some(origin_str.to_string());
            }
            continue;
        }

        let mut parts = line.split_whitespace();
        let name = match parts.next() {
            Some(n) => n.to_string(),
            None => continue,
        };

        let next = match parts.next() {
            Some(n) => n,
            None => continue,
        };

        let (ttl, class, type_str) = if let Ok(parsed_ttl) = next.parse::<u32>() {
            let class = parts.next().unwrap_or("IN");
            let type_str = parts.next().unwrap_or_default();
            (parsed_ttl, class.to_string(), type_str)
        } else if next.eq_ignore_ascii_case("IN") || next.eq_ignore_ascii_case("CH") {
            let class = next;
            let type_str = parts.next().unwrap_or_default();
            (default_ttl, class.to_string(), type_str)
        } else {
            (default_ttl, "IN".to_string(), next)
        };

        let rclass = match class.to_uppercase().as_str() {
            "IN" => DNSClass::IN,
            "CS" => DNSClass::CS,
            "CH" => DNSClass::CH,
            "HS" => DNSClass::HS,
            "NONE" => DNSClass::NONE,
            "ANY" => DNSClass::ANY,
            _ => continue,
        };

        let rtype = match type_str.to_uppercase().as_str() {
            "A" => DNSRecordType::A,
            "AAAA" => DNSRecordType::AAAA,
            "NS" => DNSRecordType::NS,
            "MX" => DNSRecordType::MX,
            "TXT" => DNSRecordType::TXT,
            "SOA" => DNSRecordType::SOA,
            "CNAME" => DNSRecordType::CNAME,
            "PTR" => DNSRecordType::PTR,
            "SRV" => DNSRecordType::SRV,
            "CAA" => DNSRecordType::CAA,
            "NAPTR" => DNSRecordType::NAPTR,
            _ => continue,
        };

        let mut value_parts: Vec<&str> = parts.collect();

        let value_str = if rtype == DNSRecordType::TXT {
            value_parts.join(" ")
        } else {
            value_parts.join(" ")
        };

        let mut record = DNSRecord {
            name: name.clone(),
            rtype: rtype.clone(),
            rclass: rclass.clone(),
            ttl,
            value: value_str,
            priority: None,
            weight: None,
            port: None,
            flags: None,
            tag: None,
            regex: None,
            replacement: None,
            order: None,
            preference: None,
        };

        match rtype {
            DNSRecordType::MX => {
                if value_parts.len() >= 2 {
                    if let Ok(prio) = value_parts[0].parse::<u16>() {
                        record.priority = Some(prio);
                        record.value = value_parts[1..].join(" ");
                    }
                }
            }
            DNSRecordType::SRV => {
                if value_parts.len() >= 4 {
                    record.priority = value_parts[0].parse().ok();
                    record.weight = value_parts[1].parse().ok();
                    record.port = value_parts[2].parse().ok();
                    record.value = value_parts[3..].join(" ");
                }
            }
            DNSRecordType::CAA => {
                if value_parts.len() >= 3 {
                    record.flags = value_parts[0].parse().ok();
                    record.tag = Some(value_parts[1].to_string());
                    record.value = value_parts[2..].join(" ");
                }
            }
            DNSRecordType::NAPTR => {
                if value_parts.len() >= 5 {
                    record.order = value_parts[0].parse().ok();
                    record.preference = value_parts[1].parse().ok();
                    record.flags = Some(value_parts[2].chars().next().unwrap_or_default() as u8);
                    record.regex = Some(value_parts[3].to_string());
                    record.replacement = Some(value_parts[4].to_string());
                }
            }
            _ => {}
        }

        match rtype {
            DNSRecordType::SOA => zone.soa = Some(record),
            _ => zone.records.entry(name).or_default().push(record),
        }
    }

    Ok(zone)
}
