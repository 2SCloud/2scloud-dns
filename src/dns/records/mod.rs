pub(crate) use crate::dns::records::q_class::DNSClass;
pub(crate) use crate::dns::records::q_type::DNSRecordType;

pub(crate) mod q_class;
pub(crate) mod q_name;
pub(crate) mod q_type;

#[derive(Debug)]
pub struct ResourceRecord {
    pub name: String,           // domain name
    pub rr_type: DNSRecordType, // record type
    pub class: DNSClass,        // record class
    pub ttl: u32,               // time-to-live
    pub rdlength: u16,          // length of rdata
    pub rdata: Vec<u8>,         // resource data
}
