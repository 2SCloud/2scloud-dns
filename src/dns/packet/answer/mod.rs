use crate::dns::records::{DNSClass, DNSRecordType};
use crate::dns::records::q_name::parse_qname;
use crate::exceptions::SCloudException;

#[derive(Debug)]
pub struct AnswerSection {
    pub name: String,
    pub r_type: DNSRecordType,
    pub r_class: DNSClass,
    pub ttl: u32,
    pub rdlength: u16,
    pub rdata: Vec<u8>,
}

impl AnswerSection {
    /// Serialize answer section into bytes (simple form, no compression)
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        // Encode NAME
        for label in self.name.split('.') {
            buf.push(label.len() as u8);
            buf.extend_from_slice(label.as_bytes());
        }
        buf.push(0x00);

        let rtype_u16 = u16::try_from(self.r_type)
            .expect("Failed to convert r_type into u16");
        buf.extend_from_slice(&rtype_u16.to_be_bytes());

        let rclass_u16 = u16::from(self.r_class);
        buf.extend_from_slice(&rclass_u16.to_be_bytes());

        buf.extend_from_slice(&self.ttl.to_be_bytes());
        buf.extend_from_slice(&self.rdlength.to_be_bytes());
        buf.extend_from_slice(&self.rdata);

        buf
    }

    /// Deserialize one AnswerSection and return (section, consumed_bytes)
    pub fn from_bytes(buf: &[u8]) -> Result<(AnswerSection, usize), SCloudException> {
        let (name, consumed_name) = parse_qname(buf)?;
        let mut pos = consumed_name;

        if buf.len() < pos + 10 {
            return Err(SCloudException::SCLOUD_ANSWER_DESERIALIZATION_FAILED);
        }

        let r_type = DNSRecordType::try_from(u16::from_be_bytes([buf[pos], buf[pos+1]]))?;
        pos += 2;

        let r_class = DNSClass::from(u16::from_be_bytes([buf[pos], buf[pos+1]]));
        pos += 2;

        let ttl = u32::from_be_bytes([buf[pos], buf[pos+1], buf[pos+2], buf[pos+3]]);
        pos += 4;

        let rdlength = u16::from_be_bytes([buf[pos], buf[pos+1]]);
        pos += 2;

        if buf.len() < pos + rdlength as usize {
            return Err(SCloudException::SCLOUD_ANSWER_DESERIALIZATION_FAILED);
        }

        let rdata = buf[pos .. pos + rdlength as usize].to_vec();
        pos += rdlength as usize;

        Ok((
            AnswerSection {
                name,
                r_type,
                r_class,
                ttl,
                rdlength,
                rdata,
            },
            pos
        ))
    }
}
