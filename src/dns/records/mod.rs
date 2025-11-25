use crate::dns::packet::question::qclass::DNSClass;

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

#[derive(Debug)]
pub struct ResourceRecord {
    pub name: String,           // domain name
    pub rr_type: DNSRecordType, // record type
    pub class: DNSClass,        // record class
    pub ttl: u32,               // time-to-live
    pub rdlength: u16,          // length of rdata
    pub rdata: Vec<u8>,         // resource data
}
