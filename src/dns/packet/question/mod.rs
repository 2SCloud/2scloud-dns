use crate::dns::q_class::DNSClass;
use crate::dns::q_name::parse_qname;
use crate::dns::q_type::DNSRecordType;
use crate::exceptions::SCloudException;

/// A DNS Question section
#[derive(Debug, PartialEq, Clone)]
pub struct QuestionSection {
    pub q_name: String,
    pub q_type: DNSRecordType,
    pub q_class: DNSClass,
}

impl QuestionSection {
    /// Serialize the DNS question section into a byte array
    ///
    /// # Exemple :
    /// ```
    /// use crate::dns::packet::question::QuestionSection;
    /// use crate::dns::q_type::DNSRecordType;
    /// use crate::dns::q_class::DNSClass;
    ///
    /// let question = QuestionSection {
    ///     q_name: "example.com".to_string(),
    ///     q_type: DNSRecordType::A,
    ///     q_class: DNSClass::IN,
    /// };
    ///
    /// let bytes = question.to_bytes().unwrap();
    ///
    /// // QNAME + QTYPE + QCLASS
    /// assert!(bytes.len() > 6);
    /// ```
    pub fn to_bytes(&self) -> Result<Vec<u8>, SCloudException> {
        let mut buf = Vec::with_capacity(self.q_name.len() + 5);

        // Encode QNAME
        for label in self.q_name.split('.') {
            let len = label.len();
            if len > 63 {
                return Err(SCloudException::SCLOUD_QUESTION_SERIALIZATION_FAILED_QNAME_TOO_LONG);
            }
            buf.push(len as u8);
            buf.extend_from_slice(label.as_bytes());
        }
        buf.push(0x00);

        let q_type_u16 =
            u16::try_from(self.q_type).expect("Cannot convert QuestionSection q_type to u16");
        let q_class_u16 = u16::try_from(self.q_class).unwrap();

        buf.extend_from_slice(&q_type_u16.to_be_bytes());
        buf.extend_from_slice(&q_class_u16.to_be_bytes());

        Ok(buf)
    }

    /// Deserialize the DNS question section from a byte array
    ///
    /// # Exemple :
    /// ```
    /// use crate::dns::packet::question::QuestionSection;
    /// use crate::dns::q_type::DNSRecordType;
    /// use crate::dns::q_class::DNSClass;
    ///
    /// // example.com A IN
    /// let raw_question: Vec<u8> = vec![
    ///     0x07, b'e', b'x', b'a', b'm', b'p', b'l', b'e',
    ///     0x03, b'c', b'o', b'm',
    ///     0x00,       // End of QNAME
    ///     0x00, 0x01, // QTYPE = A
    ///     0x00, 0x01, // QCLASS = IN
    /// ];
    ///
    /// let (question, consumed) = QuestionSection::from_bytes(&raw_question, 0).unwrap();
    ///
    /// assert_eq!(question.q_name, "example.com");
    /// assert_eq!(question.q_type, DNSRecordType::A);
    /// assert_eq!(question.q_class, DNSClass::IN);
    /// assert_eq!(consumed, raw_question.len());
    /// ```
    pub fn from_bytes(
        buf: &[u8],
        offset: usize,
    ) -> Result<(QuestionSection, usize), SCloudException> {
        let (q_name, mut pos) = parse_qname(buf, offset).unwrap();

        if buf.len() < pos + 4 {
            return Err(SCloudException::SCLOUD_QUESTION_DESERIALIZATION_FAILED);
        }

        let q_type = DNSRecordType::try_from(u16::from_be_bytes([buf[pos], buf[pos + 1]])).unwrap();

        let q_class = DNSClass::try_from(u16::from_be_bytes([buf[pos + 2], buf[pos + 3]])).unwrap();

        pos += 4;

        Ok((
            QuestionSection {
                q_name,
                q_type,
                q_class,
            },
            pos - offset,
        ))
    }
}
