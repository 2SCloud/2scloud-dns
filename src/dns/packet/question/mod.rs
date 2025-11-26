use crate::dns::records::{DNSClass, DNSRecordType};
use crate::dns::records::q_name::parse_qname;
use crate::exceptions::SCloudException;

/// A DNS Question section
#[derive(Debug)]
pub struct QuestionSection {
    pub q_name: String,
    pub q_type: DNSRecordType,
    pub q_class: DNSClass,
}

impl QuestionSection {
    /// Serialize the DNS question section into a byte array
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.q_name.len() + 5);

        // Encode QNAME
        for label in self.q_name.split('.') {
            let len = label.len();
            if len > 63 {
                panic!("Label too long for DNS: {}", label);
            }
            buf.push(len as u8);
            buf.extend_from_slice(label.as_bytes());
        }
        buf.push(0x00);

        let q_type_u16 = u16::try_from(self.q_type)
            .expect("Cannot convert QuestionSection q_type to u16");
        let q_class_u16 = u16::from(self.q_class);

        buf.extend_from_slice(&q_type_u16.to_be_bytes());
        buf.extend_from_slice(&q_class_u16.to_be_bytes());

        buf
    }

    /// Deserialize the DNS question section from a byte array
    pub fn from_bytes(buf: &[u8]) -> Result<(QuestionSection, usize), SCloudException> {
        let (q_name, consumed_qname) = parse_qname(buf)?;
        let mut pos = consumed_qname;

        if buf.len() < pos + 4 {
            return Err(SCloudException::SCLOUD_QUESTION_DESERIALIZATION_FAILED);
        }

        let q_type = DNSRecordType::try_from(u16::from_be_bytes([buf[pos], buf[pos + 1]]))?;
        let q_class = DNSClass::from(u16::from_be_bytes([buf[pos + 2], buf[pos + 3]]));

        pos += 4;

        Ok((
            QuestionSection {
                q_name,
                q_type,
                q_class,
            },
            pos,
        ))
    }
}