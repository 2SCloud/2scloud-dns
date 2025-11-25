pub mod qclass;

use crate::dns::packet::question::qclass::DNSClass;
use crate::dns::records;

#[derive(Debug)]
pub struct QuestionSection {
    q_name: String,
    q_type: records::DNSRecordType,
    q_class: DNSClass,
}
