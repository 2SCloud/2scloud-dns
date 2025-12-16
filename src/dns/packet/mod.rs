use crate::dns::packet::additional::AdditionalSection;
use crate::dns::packet::answer::AnswerSection;
use crate::dns::packet::authority::AuthoritySection;
use crate::dns::packet::header::Header;
use crate::dns::packet::question::QuestionSection;
use crate::exceptions::SCloudException;
use rand::random;
use std::io::Read;

pub(crate) mod additional;
pub(crate) mod answer;
pub(crate) mod authority;
pub(crate) mod header;
pub(crate) mod question;

#[derive(Debug, PartialEq)]
pub struct DNSPacket {
    pub header: Header,
    pub questions: Vec<QuestionSection>,
    pub answers: Vec<AnswerSection>,
    pub authorities: Vec<AuthoritySection>,
    pub additionals: Vec<AdditionalSection>,
}

impl DNSPacket {
    /// Deserialize the DNS packet from a byte array
    pub fn from_bytes(buf: &[u8]) -> Result<DNSPacket, SCloudException> {
        let mut pos = 0;

        let header = Header::from_bytes(&buf[pos..])?;
        pos += Header::DNS_HEADER_LEN;

        let mut questions = Vec::new();
        for _ in 0..header.qdcount {
            let (q, consumed) = QuestionSection::from_bytes(&buf, pos)?;
            pos += consumed;
            questions.push(q);
        }

        let mut answers = Vec::new();
        for _ in 0..header.ancount {
            let (ans, consumed) = AnswerSection::from_bytes(&buf, pos)?;
            pos += consumed;
            answers.push(ans);
        }

        let mut authorities = Vec::new();
        for _ in 0..header.nscount {
            let (ns, consumed) = AuthoritySection::from_bytes(buf, pos)?;
            pos += consumed;
            authorities.push(ns);
        }

        let mut additionals = Vec::new();
        for _ in 0..header.arcount {
            let (add, consumed) = AdditionalSection::from_bytes(&buf, pos)?;
            pos += consumed;
            additionals.push(add);
        }

        Ok(DNSPacket {
            header,
            questions,
            answers,
            authorities,
            additionals,
        })
    }

    // TODO: make the others parts of the DNSPacket
    /// Serialize the DNS packet into a byte array
    pub fn to_bytes(&self) -> Result<Vec<u8>, SCloudException> {
        let mut bytes = Vec::new();

        if let Err(_) = self.header.to_bytes() {
            return Err(SCloudException::SCLOUD_HEADER_BYTES_EMPTY);
        }
        bytes.extend_from_slice(&self.header.to_bytes()?);

        for q in &self.questions {
            bytes.extend_from_slice(&q.to_bytes()?);
        }

        for ans in &self.answers {
            bytes.extend_from_slice(&ans.to_bytes()?);
        }

        for auth in &self.authorities {
            bytes.extend_from_slice(&auth.to_bytes()?);
        }

        for add in &self.additionals {
            bytes.extend_from_slice(&add.to_bytes()?);
        }

        Ok(bytes)
    }

    /// Receive a QuestionSection, and return an AnswerSection
    pub fn new_query(question_section: Vec<QuestionSection>) -> DNSPacket {
        DNSPacket {
            header: Header {
                id: random::<u16>(),
                qr: false,
                opcode: 0,
                aa: false,
                tc: false,
                rd: true,
                ra: false,
                z: 0,
                rcode: 0,
                qdcount: question_section.len() as u16,
                ancount: 0,
                nscount: 0,
                arcount: 0,
            },
            questions: question_section,
            answers: vec![],
            authorities: vec![],
            additionals: vec![],
        }
    }
}
