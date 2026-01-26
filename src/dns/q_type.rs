use crate::exceptions::SCloudException;
use std::convert::TryFrom;
use strum_macros::EnumIter;

/// DNS Resource Record Types (QTYPE / TYPE).
///
/// This enum represents DNS record types as defined by IANA and
/// various RFCs (RFC 1035, RFC 4034, RFC 6895, etc.).
///
/// # Design notes
/// - DNS is an extensible protocol by design.
/// - Unknown record types MUST be preserved and forwarded as-is.
/// - Therefore, unknown types are represented using `Unknown(u16)`.
///
/// This enum supports:
/// - Conversion from the on-the-wire `u16` representation
/// - Conversion back to `u16` for packet serialization
///
/// This guarantees that the resolver does not break when encountering
/// newer or unsupported DNS record types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
pub(crate) enum DNSRecordType {
    A,
    AAAA,
    AFSDB,
    APL,
    CAA,
    CDNSKEY,
    CDS,
    CERT,
    CNAME,
    CSYNC,
    DHCID,
    DLV,
    DNAME,
    DNSKEY,
    DS,
    EUI48,
    EUI64,
    HINFO,
    HIP,
    HTTPS,
    IPSECKEY,
    KEY,
    KX,
    LOC,
    MX,
    NAPTR,
    NS,
    NSEC,
    NSEC3,
    NSEC3PARAM,
    OPENPGPKEY,
    PTR,
    RP,
    RRSIG,
    SIG,
    SMIMEA,
    SOA,
    SRV,
    SSHFP,
    SVCB,
    TA,
    TKEY,
    TLSA,
    TSIG,
    TXT,
    URI,
    ZONEMD,

    /// Unknown or unsupported DNS record type.
    ///
    /// The contained `u16` value is the raw QTYPE as received on the wire.
    /// This variant ensures DNS extensibility is preserved.
    Unknown(u16),
}

impl TryFrom<u16> for DNSRecordType {
    type Error = SCloudException;

    /// Convert a DNS QTYPE value (`u16`) into a `DNSRecordType`.
    ///
    /// Unknown values are preserved using `DNSRecordType::Unknown`.
    fn try_from(v: u16) -> Result<Self, Self::Error> {
        Ok(match v {
            1 => DNSRecordType::A,
            28 => DNSRecordType::AAAA,
            18 => DNSRecordType::AFSDB,
            42 => DNSRecordType::APL,
            257 => DNSRecordType::CAA,
            60 => DNSRecordType::CDNSKEY,
            59 => DNSRecordType::CDS,
            37 => DNSRecordType::CERT,
            5 => DNSRecordType::CNAME,
            62 => DNSRecordType::CSYNC,
            49 => DNSRecordType::DHCID,
            32769 => DNSRecordType::DLV,
            39 => DNSRecordType::DNAME,
            48 => DNSRecordType::DNSKEY,
            43 => DNSRecordType::DS,
            108 => DNSRecordType::EUI48,
            109 => DNSRecordType::EUI64,
            13 => DNSRecordType::HINFO,
            55 => DNSRecordType::HIP,
            65 => DNSRecordType::HTTPS,
            45 => DNSRecordType::IPSECKEY,
            25 => DNSRecordType::KEY,
            36 => DNSRecordType::KX,
            29 => DNSRecordType::LOC,
            15 => DNSRecordType::MX,
            35 => DNSRecordType::NAPTR,
            2 => DNSRecordType::NS,
            47 => DNSRecordType::NSEC,
            50 => DNSRecordType::NSEC3,
            51 => DNSRecordType::NSEC3PARAM,
            61 => DNSRecordType::OPENPGPKEY,
            12 => DNSRecordType::PTR,
            17 => DNSRecordType::RP,
            46 => DNSRecordType::RRSIG,
            24 => DNSRecordType::SIG,
            53 => DNSRecordType::SMIMEA,
            6 => DNSRecordType::SOA,
            33 => DNSRecordType::SRV,
            44 => DNSRecordType::SSHFP,
            64 => DNSRecordType::SVCB,
            32768 => DNSRecordType::TA,
            249 => DNSRecordType::TKEY,
            52 => DNSRecordType::TLSA,
            250 => DNSRecordType::TSIG,
            16 => DNSRecordType::TXT,
            256 => DNSRecordType::URI,
            63 => DNSRecordType::ZONEMD,
            _ => DNSRecordType::Unknown(v),
        })
    }
}

impl TryFrom<DNSRecordType> for u16 {
    type Error = SCloudException;

    /// Convert a `DNSRecordType` into its DNS wire format (`u16`).
    ///
    /// Unknown record types are returned as-is to preserve DNS extensibility.
    fn try_from(rt: DNSRecordType) -> Result<Self, Self::Error> {
        Ok(match rt {
            DNSRecordType::A => 1,
            DNSRecordType::AAAA => 28,
            DNSRecordType::AFSDB => 18,
            DNSRecordType::APL => 42,
            DNSRecordType::CAA => 257,
            DNSRecordType::CDNSKEY => 60,
            DNSRecordType::CDS => 59,
            DNSRecordType::CERT => 37,
            DNSRecordType::CNAME => 5,
            DNSRecordType::CSYNC => 62,
            DNSRecordType::DHCID => 49,
            DNSRecordType::DLV => 32769,
            DNSRecordType::DNAME => 39,
            DNSRecordType::DNSKEY => 48,
            DNSRecordType::DS => 43,
            DNSRecordType::EUI48 => 108,
            DNSRecordType::EUI64 => 109,
            DNSRecordType::HINFO => 13,
            DNSRecordType::HIP => 55,
            DNSRecordType::HTTPS => 65,
            DNSRecordType::IPSECKEY => 45,
            DNSRecordType::KEY => 25,
            DNSRecordType::KX => 36,
            DNSRecordType::LOC => 29,
            DNSRecordType::MX => 15,
            DNSRecordType::NAPTR => 35,
            DNSRecordType::NS => 2,
            DNSRecordType::NSEC => 47,
            DNSRecordType::NSEC3 => 50,
            DNSRecordType::NSEC3PARAM => 51,
            DNSRecordType::OPENPGPKEY => 61,
            DNSRecordType::PTR => 12,
            DNSRecordType::RP => 17,
            DNSRecordType::RRSIG => 46,
            DNSRecordType::SIG => 24,
            DNSRecordType::SMIMEA => 53,
            DNSRecordType::SOA => 6,
            DNSRecordType::SRV => 33,
            DNSRecordType::SSHFP => 44,
            DNSRecordType::SVCB => 64,
            DNSRecordType::TA => 32768,
            DNSRecordType::TKEY => 249,
            DNSRecordType::TLSA => 52,
            DNSRecordType::TSIG => 250,
            DNSRecordType::TXT => 16,
            DNSRecordType::URI => 256,
            DNSRecordType::ZONEMD => 63,
            DNSRecordType::Unknown(v) => v,
        })
    }
}

impl TryFrom<&[u8; 2]> for DNSRecordType {
    type Error = SCloudException;

    /// Convert a 2-byte DNS wire representation into a `DNSRecordType`.
    fn try_from(bytes: &[u8; 2]) -> Result<Self, Self::Error> {
        let v = u16::from_be_bytes(*bytes);
        DNSRecordType::try_from(v)
    }
}
