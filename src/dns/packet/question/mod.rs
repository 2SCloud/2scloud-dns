use crate::dns::packet::header::Header;
use crate::dns::records;
use crate::dns::records::{DNSClass, DNSRecordType};
use crate::exceptions::SCloudException;
use crate::utils::ErrorCondition;

#[derive(Debug)]
pub struct QuestionSection {
    q_name: String,
    q_type: records::DNSRecordType,
    q_class: DNSClass,
}

impl QuestionSection {
    const DNS_QUESTION_SECTION_LEN: usize = 5;

    /// Serialize the DNS question section into a byte array
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(QuestionSection::DNS_QUESTION_SECTION_LEN + self.q_name.len());

        buf.extend_from_slice(&self.q_name.into_bytes());
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

    /// Deserialize the DNS question section from a byte array
    pub fn from_bytes(buf: &[u8]) -> Result<QuestionSection, SCloudException> {
        if buf[11..].len() < QuestionSection::DNS_QUESTION_SECTION_LEN {
            return Err(SCloudException::SCLOUD_QUESTION_DESERIALIZATION_FAILED);
        }

        let q_name = until_null(&buf[11..])?;
        let pos = 11 + q_name.1;

        let q_type_bytes: &[u8; 2] = buf[pos..pos+2].try_into().unwrap();
        let q_class_bytes: &[u8; 2] = buf[pos+2..pos+4].try_into().unwrap();

        Ok(QuestionSection {
            q_name: String::from_utf8(Vec::from(q_name.0))
                .map_err(|_| SCloudException::SCLOUD_QUESTION_DESERIALIZATION_FAILED)?,
            q_type: DNSRecordType::from(q_type_bytes),
            q_class: DNSClass::from(q_class_bytes),
        })
    }
}

fn until_null(buf: &[u8]) -> Result<(&[u8], usize), SCloudException> {
    match buf.iter().position(|&b| b == 0x00) {
        Some(pos) => Ok((&buf[..pos], buf[..pos].len())),
        None => Err(SCloudException::SCLOUD_QUESTION_IMPOSSIBLE_PARSE_QNAME),
    }
}