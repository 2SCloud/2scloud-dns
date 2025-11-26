use crate::dns::packet::additional::AdditionalSection;
use crate::dns::packet::answer::AnswerSection;
use crate::dns::packet::authority::AuthoritySection;
use crate::dns::packet::header::Header;
use crate::dns::packet::question::QuestionSection;
use crate::exceptions::SCloudException;

pub mod header;
pub(crate) mod question;
mod answer;
mod additional;
mod authority;

pub struct DNSPacket {
    pub header: Header,
    pub questions: Vec<QuestionSection>,
    pub answers: Vec<AnswerSection>,
    pub authorities: Vec<AuthoritySection>,
    pub additionals: Vec<AdditionalSection>,
}

impl DNSPacket {
    pub fn from_bytes(buf: &[u8]) -> Result<DNSPacket, SCloudException> {
        let mut pos = 0;

        let header = Header::from_bytes(&buf[pos..])?;
        pos += Header::DNS_HEADER_LEN;

        let mut questions = Vec::new();
        for _ in 0..header.qdcount {
            let (q, consumed) = QuestionSection::from_bytes(&buf[pos..])?;
            pos += consumed;
            questions.push(q);
        }

        let mut answers = Vec::new();
        for _ in 0..header.ancount {
            let (ans, consumed) = AnswerSection::from_bytes(&buf[pos..])?;
            pos += consumed;
            answers.push(ans);
        }

        let mut authorities = Vec::new();
        for _ in 0..header.nscount {
            let (ns, consumed) = AuthoritySection::from_bytes(&buf[pos..])?;
            pos += consumed;
            authorities.push(ns);
        }

        let mut additionals = Vec::new();
        for _ in 0..header.arcount {
            let (add, consumed) = AdditionalSection::from_bytes(&buf[pos..])?;
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
}
