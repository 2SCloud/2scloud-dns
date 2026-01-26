use crate::dns::q_class::DNSClass;
use crate::dns::q_name::parse_qname;
use crate::dns::q_type::DNSRecordType;
use crate::exceptions::SCloudException;

#[derive(PartialEq, Debug)]
pub(crate) struct AdditionalSection {
    pub(crate) q_name: String,
    pub(crate) q_type: DNSRecordType,
    pub(crate) q_class: DNSClass,
    pub(crate) ttl: u32,
    pub rdlength: u16,
    pub rdata: Vec<u8>,
}

impl AdditionalSection {
    /// Deserialize one AdditionalSection and return (section, consumed_bytes)
    ///
    /// # Exemple :
    /// ```
    /// use crate::dns::packet::additional::AdditionalSection;
    /// use crate::dns::q_type::DNSRecordType;
    /// use crate::dns::q_class::DNSClass;
    ///
    /// // Record A in additional section (ns1.example.com → 192.0.2.1)
    /// let raw_additional: Vec<u8> = vec![
    ///     0x03, b'n', b's', b'1',
    ///     0x07, b'e', b'x', b'a', b'm', b'p', b'l', b'e',
    ///     0x03, b'c', b'o', b'm',
    ///     0x00,             // End of QNAME
    ///     0x00, 0x01,       // TYPE = A
    ///     0x00, 0x01,       // CLASS = IN
    ///     0x00, 0x00, 0x01, 0x2c, // TTL = 300
    ///     0x00, 0x04,       // RDLENGTH = 4
    ///     192, 0, 2, 1,     // RDATA
    /// ];
    ///
    /// let (additional, consumed) =
    ///     AdditionalSection::from_bytes(&raw_additional, 0).unwrap();
    ///
    /// assert_eq!(additional.q_name, "ns1.example.com");
    /// assert_eq!(additional.q_type, DNSRecordType::A);
    /// assert_eq!(additional.q_class, DNSClass::IN);
    /// assert_eq!(additional.ttl, 300);
    /// assert_eq!(additional.rdata, vec![192, 0, 2, 1]);
    /// assert_eq!(consumed, raw_additional.len());
    /// ```
    pub(crate) fn from_bytes(
        buf: &[u8],
        offset: usize,
    ) -> Result<(AdditionalSection, usize), SCloudException> {
        let (q_name, consumed_name) = parse_qname(buf, offset).unwrap();
        let mut pos = consumed_name;

        if buf.len() < pos + 10 {
            return Err(SCloudException::SCLOUD_ADDITIONAL_DESERIALIZATION_FAILED_BUF_TOO_SHORT);
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
                SCloudException::SCLOUD_ADDITIONAL_DESERIALIZATION_FAILED_RDATA_OUT_OF_BOUNDS,
            );
        }

        let rdata = buf[pos..pos + rdlength as usize].to_vec();
        pos += rdlength as usize;

        Ok((
            AdditionalSection {
                q_name,
                q_type,
                q_class,
                ttl,
                rdlength,
                rdata,
            },
            pos,
        ))
    }

    /// Serialize the AdditionalSection into bytes
    ///
    /// # Exemple :
    /// ```
    /// use crate::dns::packet::additional::AdditionalSection;
    /// use crate::dns::q_type::DNSRecordType;
    /// use crate::dns::q_class::DNSClass;
    ///
    /// let additional = AdditionalSection {
    ///     q_name: "ns1.example.com".to_string(),
    ///     q_type: DNSRecordType::A,
    ///     q_class: DNSClass::IN,
    ///     ttl: 300,
    ///     rdlength: 4,
    ///     rdata: vec![192, 0, 2, 1],
    /// };
    ///
    /// let bytes = additional.to_bytes().unwrap();
    ///
    /// // NAME + TYPE + CLASS + TTL + RDLENGTH + RDATA
    /// assert!(bytes.len() > 20);
    /// ```
    pub(crate) fn to_bytes(&self) -> Result<Vec<u8>, SCloudException> {
        let mut buf: Vec<u8> = Vec::new();

        for label in self.q_name.split('.') {
            let len = label.len();
            if len > 63 {
                return Err(
                    SCloudException::SCLOUD_ADDITIONAL_DESERIALIZATION_FAILED_QNAME_TOO_LONG,
                );
            }
            buf.push(len as u8);
            buf.extend_from_slice(label.as_bytes());
        }
        buf.push(0x00);

        let qtype_u16 =
            u16::try_from(self.q_type).expect("Cannot convert AdditionalSection q_type to u16");
        buf.extend_from_slice(&qtype_u16.to_be_bytes());

        let qclass_u16 = u16::try_from(self.q_class).unwrap();
        buf.extend_from_slice(&qclass_u16.to_be_bytes());

        buf.extend_from_slice(&self.ttl.to_be_bytes());
        buf.extend_from_slice(&self.rdlength.to_be_bytes());
        buf.extend_from_slice(&self.rdata);

        Ok(buf)
    }
}
