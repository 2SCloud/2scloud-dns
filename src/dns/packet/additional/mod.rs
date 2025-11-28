use std::ops::Add;
use crate::dns::records;
use crate::dns::records::DNSClass;
use crate::exceptions::SCloudException;

pub(crate) struct AdditionalSection {
    q_name: String,
    q_type: records::DNSRecordType,
    q_class: DNSClass,
    ttl: u32,
    pub rdlength: u16,
    pub rdata: Vec<u8>,
}

impl AdditionalSection {

    pub(crate) fn from_bytes(buf: &[u8]) -> Result<(AdditionalSection, usize), SCloudException> {

    }

    pub(crate) fn to_bytes(&self) -> Vec<u8> {

    }

}