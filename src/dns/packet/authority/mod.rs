use crate::dns::packet::answer::AnswerSection;
use crate::dns::records;
use crate::dns::records::{DNSClass, DNSRecordType};
use crate::dns::records::q_name::parse_qname;
use crate::exceptions::SCloudException;

pub(crate) struct AuthoritySection {
    q_name: String,
    q_type: records::DNSRecordType,
    q_class: DNSClass,
    ttl: u32,
    pub rdlength: u16,
    pub rdata: Vec<u8>,
}

impl AuthoritySection {

    pub(crate) fn from_bytes(buf: &[u8]) -> Result<(AuthoritySection, usize), SCloudException> {
        let (q_name, consumed_name) = parse_qname(buf)?;
        let mut pos = consumed_name;

        if buf.len() < pos + 10 {
            return Err(SCloudException::SCLOUD_AUTHORITY_DESERIALIZATION_FAILED);
        }

        let q_type = DNSRecordType::try_from(u16::from_be_bytes([buf[pos], buf[pos+1]]))?;
        pos += 2;

        let q_class = DNSClass::from(u16::from_be_bytes([buf[pos], buf[pos+1]]));
        pos += 2;

        let ttl = u32::from_be_bytes([buf[pos], buf[pos+1], buf[pos+2], buf[pos+3]]);
        pos += 4;

        let rdlength = u16::from_be_bytes([buf[pos], buf[pos+1]]);
        pos += 2;

        if buf.len() < pos + rdlength as usize {
            return Err(SCloudException::SCLOUD_AUTHORITY_DESERIALIZATION_FAILED);
        }

        let rdata = buf[pos .. pos + rdlength as usize].to_vec();
        pos += rdlength as usize;

        Ok((
            AuthoritySection {
                q_name,
                q_type,
                q_class,
                ttl,
                rdlength,
                rdata,
            },
            pos
        ))
    }

    pub(crate) fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();

        for label in self.q_name.split('.') {
            let len = label.len();
            if len > 63 {
                panic!("Label too long for DNS: {}", label);
            }
            buf.push(len as u8);
            buf.extend_from_slice(label.as_bytes());
        }
        buf.push(0x00);

        let qtype_u16 = u16::try_from(self.q_type)
            .expect("Cannot convert AuthoritySection q_type to u16");
        buf.extend_from_slice(&qtype_u16.to_be_bytes());

        let qclass_u16 = u16::from(self.q_class);
        buf.extend_from_slice(&qclass_u16.to_be_bytes());

        buf.extend_from_slice(&self.ttl.to_be_bytes());
        buf.extend_from_slice(&self.rdlength.to_be_bytes());
        buf.extend_from_slice(&self.rdata);

        buf
    }

}
