mod cache;
pub(crate) mod packet;
pub(crate) mod records;

use crate::dns::packet::header::Header;
use crate::dns::packet::question::QuestionSection;
use crate::dns::records::ResourceRecord;

#[derive(Debug)]
pub struct DNSMessage {
    pub header: Header,
    pub questions: Vec<QuestionSection>,
    pub answers: Vec<ResourceRecord>,
    pub authorities: Vec<ResourceRecord>,
    pub additionals: Vec<ResourceRecord>,
}
