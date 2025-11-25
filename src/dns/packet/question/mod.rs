use crate::dns::records;
use crate::dns::records::DNSClass;

#[derive(Debug)]
pub struct QuestionSection {
    q_name: String,
    q_type: records::DNSRecordType,
    q_class: DNSClass,
}

