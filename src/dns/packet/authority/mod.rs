use crate::dns::packet::answer::AnswerSection;
use crate::dns::records;
use crate::dns::records::q_name::parse_qname;
use crate::dns::records::{DNSClass, DNSRecordType};
use crate::exceptions::SCloudException;

#[derive(PartialEq, Debug)]
pub(crate) struct AuthoritySection {
    pub(crate) q_name: String,
    pub(crate) q_type: DNSRecordType,
    pub(crate) q_class: DNSClass,
    pub(crate) ttl: u32,
    pub(crate) ns_name: String,
}

impl AuthoritySection {
    pub(crate) fn from_bytes(
        buf: &[u8],
        offset: usize,
    ) -> Result<(AuthoritySection, usize), SCloudException> {
        let (q_name, mut pos) = parse_qname(buf, offset).unwrap();

        if buf.len() < pos + 10 {
            return Err(SCloudException::SCLOUD_AUTHORITY_DESERIALIZATION_FAILED_BUF_TOO_SHORT);
        }

        let q_type = DNSRecordType::try_from(u16::from_be_bytes([buf[pos], buf[pos + 1]])).unwrap();
        pos += 2;

        let q_class = DNSClass::try_from(u16::from_be_bytes([buf[pos], buf[pos + 1]])).unwrap();
        pos += 2;

        let ttl = u32::from_be_bytes([buf[pos], buf[pos + 1], buf[pos + 2], buf[pos + 3]]);
        pos += 4;

        let rdlength = u16::from_be_bytes([buf[pos], buf[pos + 1]]);
        pos += 2;

        if buf.len() < pos + rdlength as usize {
            return Err(
                SCloudException::SCLOUD_AUTHORITY_DESERIALIZATION_FAILED_RDATA_OUT_OF_BOUNDS,
            );
        }

        let (ns_name, _) = parse_qname(buf, pos).unwrap();

        pos += rdlength as usize;

        Ok((
            AuthoritySection {
                q_name,
                q_type,
                q_class,
                ttl,
                ns_name,
            },
            pos - offset,
        ))
    }

    pub(crate) fn to_bytes(&self) -> Result<Vec<u8>, SCloudException> {
        let mut buf = Vec::new();

        for label in self.q_name.split('.') {
            buf.push(label.len() as u8);
            buf.extend_from_slice(label.as_bytes());
        }
        buf.push(0x00);

        let qtype_u16 =
            u16::try_from(self.q_type).expect("Cannot convert AuthoritySection q_type to u16");
        buf.extend_from_slice(&qtype_u16.to_be_bytes());

        let qclass_u16 = u16::try_from(self.q_class).unwrap();
        buf.extend_from_slice(&qclass_u16.to_be_bytes());

        buf.extend_from_slice(&self.ttl.to_be_bytes());

        let mut rdata = Vec::new();
        for label in self.ns_name.split('.') {
            rdata.push(label.len() as u8);
            rdata.extend_from_slice(label.as_bytes());
        }
        rdata.push(0x00);

        buf.extend_from_slice(&(rdata.len() as u16).to_be_bytes());
        buf.extend_from_slice(&rdata);

        Ok(buf)
    }
}
