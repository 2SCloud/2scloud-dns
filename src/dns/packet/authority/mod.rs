use crate::dns::q_class::DNSClass;
use crate::dns::q_name::parse_qname;
use crate::dns::q_type::DNSRecordType;
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
    /// Deserialize one AuthoritySection (NS record) and return (section, consumed_bytes)
    ///
    /// # Exemple :
    /// ```
    /// use crate::dns::packet::authority::AuthoritySection;
    /// use crate::dns::q_type::DNSRecordType;
    /// use crate::dns::q_class::DNSClass;
    ///
    /// // example.com NS ns1.example.com
    /// let raw_authority: Vec<u8> = vec![
    ///     0x07, b'e', b'x', b'a', b'm', b'p', b'l', b'e',
    ///     0x03, b'c', b'o', b'm',
    ///     0x00,             // End of QNAME
    ///     0x00, 0x02,       // TYPE = NS
    ///     0x00, 0x01,       // CLASS = IN
    ///     0x00, 0x00, 0x0e, 0x10, // TTL = 3600
    ///     0x00, 0x11,       // RDLENGTH = 17
    ///     0x03, b'n', b's', b'1',
    ///     0x07, b'e', b'x', b'a', b'm', b'p', b'l', b'e',
    ///     0x03, b'c', b'o', b'm',
    ///     0x00,             // End of NSNAME
    /// ];
    ///
    /// let (authority, consumed) =
    ///     AuthoritySection::from_bytes(&raw_authority, 0).unwrap();
    ///
    /// assert_eq!(authority.q_name, "example.com");
    /// assert_eq!(authority.q_type, DNSRecordType::NS);
    /// assert_eq!(authority.q_class, DNSClass::IN);
    /// assert_eq!(authority.ttl, 3600);
    /// assert_eq!(authority.ns_name, "ns1.example.com");
    /// assert_eq!(consumed, raw_authority.len());
    /// ```
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

    /// Serialize the AuthoritySection (NS record) into bytes
    ///
    /// # Exemple :
    /// ```
    /// use crate::dns::packet::authority::AuthoritySection;
    /// use crate::dns::q_type::DNSRecordType;
    /// use crate::dns::q_class::DNSClass;
    ///
    /// let authority = AuthoritySection {
    ///     q_name: "example.com".to_string(),
    ///     q_type: DNSRecordType::NS,
    ///     q_class: DNSClass::IN,
    ///     ttl: 3600,
    ///     ns_name: "ns1.example.com".to_string(),
    /// };
    ///
    /// let bytes = authority.to_bytes().unwrap();
    ///
    /// // NAME + TYPE + CLASS + TTL + RDLENGTH + RDATA
    /// assert!(bytes.len() > 20);
    /// ```
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
