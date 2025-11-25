
#[derive(Debug)]
pub struct ResourceRecord {
    pub name: String,           // domain name
    pub rr_type: DNSRecordType, // record type
    pub class: DNSClass,        // record class
    pub ttl: u32,               // time-to-live
    pub rdlength: u16,          // length of rdata
    pub rdata: Vec<u8>,         // resource data
}

#[derive(Debug)]
pub enum DNSRecordType {
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DNSClass {
    IN,
    CS,
    CH,
    HS,
    NONE,
    ANY,
    Unknown(u16),
}

impl From<u16> for DNSClass {
    fn from(v: u16) -> Self {
        match v {
            0 => DNSClass::NONE,
            1 => DNSClass::IN,
            2 => DNSClass::CS,
            3 => DNSClass::CH,
            4 => DNSClass::HS,
            255 => DNSClass::ANY,
            other => DNSClass::Unknown(other),
        }
    }
}

impl From<DNSClass> for u16 {
    fn from(c: DNSClass) -> u16 {
        match c {
            DNSClass::NONE => 0,
            DNSClass::IN => 1,
            DNSClass::CS => 2,
            DNSClass::CH => 3,
            DNSClass::HS => 4,
            DNSClass::ANY => 255,
            DNSClass::Unknown(v) => v,
        }
    }
}
