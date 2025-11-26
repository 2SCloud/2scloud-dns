use crate::dns::packet::additional::AdditionalSection;
use crate::dns::packet::answer::AnswerSection;
use crate::dns::packet::authority::AuthoritySection;
use crate::dns::packet::header::Header;
use crate::dns::packet::question::QuestionSection;
use crate::utils::ErrorCondition;

pub mod header;
pub(crate) mod question;
mod answer;
mod additional;
mod authority;

pub struct DNSPacket {
    pub header: Header,
    pub question_section: QuestionSection,
    pub answer_section: AnswerSection,
    pub authority_section: AuthoritySection,
    pub additional_section: AdditionalSection
}

impl DNSPacket {
    const DNS_PACKET_LEN: usize = 12;

    /// Serialize the DNS packet into a byte array
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(Self::DNS_PACKET_LEN);

        buf.extend_from_slice(&self.id.to_be_bytes());
        buf.push(
            (self.qr as u8) << 7
                | self.opcode << 3
                | (self.aa as u8) << 2
                | (self.tc as u8) << 1
                | self.rd as u8,
        );
        buf.push((self.ra as u8) << 7 | self.z << 4 | self.rcode);
        buf.extend_from_slice(&self.qdcount.to_be_bytes());
        buf.extend_from_slice(&self.ancount.to_be_bytes());
        buf.extend_from_slice(&self.nscount.to_be_bytes());
        buf.extend_from_slice(&self.arcount.to_be_bytes());

        buf
    }

    /// Deserialize the DNS packet from a byte array
    pub fn from_bytes(buf: &[u8]) -> Result<DNSPacket, ErrorCondition> {
        if buf.len() < Self::DNS_PACKET_LEN {
            return Err(ErrorCondition::DeserializationErr(
                "Buffer length is less than header length".to_string(),
            ));
        }

        Ok(DNSPacket {
            header: Header::from_bytes(buf)?,
            question_section: QuestionSection::from_bytes(buf)?,
            answer_section: (),
            authority_section: (),
            additional_section: (),
        })
    }
}