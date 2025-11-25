use crate::dns::packet::header::Header;
use crate::dns::records;
use crate::dns::records::DNSClass;
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

    /// Deserialize the DNS question section from a byte array
    pub fn from_bytes(buf: &[u8]) -> Result<Header, ErrorCondition> {
        if buf.len() < QuestionSection::DNS_QUESTION_SECTION_LEN {
            return Err(ErrorCondition::DeserializationErr(
                "Buffer length is less than question section length".to_string(),
            ));
        }

        Ok(Header {
            id: u16::from_be_bytes([buf[0], buf[1]]),
            qr: (buf[2] & 0b1000_0000) != 0,
            opcode: (buf[2] & 0b0111_1000) >> 3,
            aa: (buf[2] & 0b0000_0100) != 0,
            tc: (buf[2] & 0b0000_0010) != 0,
            rd: (buf[2] & 0b0000_0001) != 0,
            ra: (buf[3] & 0b1000_0000) != 0,
            z: (buf[3] & 0b0111_0000) >> 4,
            rcode: buf[3] & 0b0000_1111,
            qdcount: u16::from_be_bytes([buf[4], buf[5]]),
            ancount: u16::from_be_bytes([buf[6], buf[7]]),
            nscount: u16::from_be_bytes([buf[8], buf[9]]),
            arcount: u16::from_be_bytes([buf[10], buf[11]]),
        })
    }
}