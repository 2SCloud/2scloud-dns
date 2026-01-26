use crate::dns::q_class::DNSClass;
use crate::dns::q_name::parse_qname;
use crate::dns::q_type::DNSRecordType;
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
    ///
    /// # Exemple :
    /// ```
    /// use crate::dns::answer::AnswerSection;
    /// use crate::dns::q_type::DNSRecordType;
    /// use crate::dns::q_class::DNSClass;
    ///
    /// // A Answer for example.com → 93.184.216.34
    /// let answer = AnswerSection {
    ///     q_name: "example.com".to_string(),
    ///     r_type: DNSRecordType::A,
    ///     r_class: DNSClass::IN,
    ///     ttl: 300,
    ///     rdlength: 4,
    ///     rdata: vec![93, 184, 216, 34],
    /// };
    ///
    /// let bytes = answer.to_bytes().unwrap();
    ///
    /// // NAME + TYPE + CLASS + TTL + RDLENGTH + RDATA
    /// assert!(bytes.len() > 12);
    /// ```
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
    ///
    /// # Exemple :
    /// ```
    /// use crate::dns::answer::AnswerSection;
    /// use crate::dns::q_type::DNSRecordType;
    /// use crate::dns::q_class::DNSClass;
    ///
    /// // example.com A 93.184.216.34
    /// let raw_answer: Vec<u8> = vec![
    ///     0x07, b'e', b'x', b'a', b'm', b'p', b'l', b'e',
    ///     0x03, b'c', b'o', b'm',
    ///     0x00,             // End of NAME
    ///     0x00, 0x01,       // TYPE = A
    ///     0x00, 0x01,       // CLASS = IN
    ///     0x00, 0x00, 0x01, 0x2c, // TTL = 300
    ///     0x00, 0x04,       // RDLENGTH = 4
    ///     93, 184, 216, 34, // RDATA
    /// ];
    ///
    /// let (answer, consumed) = AnswerSection::from_bytes(&raw_answer, 0).unwrap();
    ///
    /// assert_eq!(answer.q_name, "example.com");
    /// assert_eq!(answer.r_type, DNSRecordType::A);
    /// assert_eq!(answer.r_class, DNSClass::IN);
    /// assert_eq!(answer.ttl, 300);
    /// assert_eq!(answer.rdata, vec![93, 184, 216, 34]);
    /// assert_eq!(consumed, raw_answer.len());
    /// ```
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
