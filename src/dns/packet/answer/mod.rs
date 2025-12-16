use crate::dns::records::q_name::parse_qname;
use crate::dns::records::{DNSClass, DNSRecordType};
use crate::exceptions::SCloudException;

#[derive(Debug, PartialEq)]
pub struct AnswerSection {
    pub q_name: String,
    pub r_type: DNSRecordType,
    pub r_class: DNSClass,
    pub ttl: u32,
    pub rdlength: u16,
    pub rdata: Vec<u8>,
}

impl AnswerSection {
    /// Serialize AnswerSection into bytes (simple form, no compression)
    pub fn to_bytes(&self) -> Result<Vec<u8>, SCloudException> {
        let mut buf = Vec::new();

        // Encode NAME
        for label in self.q_name.split('.') {
            let len = label.len();
            if len > 63 {
                return Err(SCloudException::SCLOUD_ANSWER_DESERIALIZATION_FAILED_LABEL_TOO_LONG);
            }
            buf.push(len as u8);
            buf.extend_from_slice(label.as_bytes());
        }
        buf.push(0x00);

        let rtype_u16 =
            u16::try_from(self.r_type).expect("Cannot convert AnswerSection q_type to u16");
        buf.extend_from_slice(&rtype_u16.to_be_bytes());

        let rclass_u16 = u16::try_from(self.r_class).unwrap();
        buf.extend_from_slice(&rclass_u16.to_be_bytes());

        buf.extend_from_slice(&self.ttl.to_be_bytes());
        buf.extend_from_slice(&self.rdlength.to_be_bytes());
        buf.extend_from_slice(&self.rdata);

        Ok(buf)
    }

    /// Deserialize one AnswerSection and return (section, consumed_bytes)
    pub(crate) fn from_bytes(
        buf: &[u8],
        offset: usize,
    ) -> Result<(AnswerSection, usize), SCloudException> {
        let (q_name, mut pos) = parse_qname(buf, offset).unwrap();

        if pos + 10 > buf.len() {
            return Err(SCloudException::SCLOUD_IMPOSSIBLE_PARSE_ANSWER_HEADER_TOO_SHORT);
        }

        let r_type = DNSRecordType::try_from(u16::from_be_bytes([buf[pos], buf[pos + 1]])).unwrap();
        pos += 2;

        let r_class = DNSClass::try_from(u16::from_be_bytes([buf[pos], buf[pos + 1]])).unwrap();
        pos += 2;

        let ttl = u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]);
        pos += 4;

        let rdlength = u16::from_be_bytes([buf[pos], buf[pos + 1]]);
        pos += 2;

        if pos + rdlength as usize > buf.len() {
            return Err(SCloudException::SCLOUD_IMPOSSIBLE_PARSE_ANSWER_RDATA_OUT_OF_BOUNDS);
        }
        let rdata = buf[pos..pos + rdlength as usize].to_vec();
        pos += rdlength as usize;

        Ok((
            AnswerSection {
                q_name,
                r_type,
                r_class,
                ttl,
                rdlength,
                rdata,
            },
            pos - offset,
        ))
    }
}
