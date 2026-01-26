use crate::exceptions::SCloudException;

#[derive(Debug, PartialEq, Default)]
pub struct Header {
    pub id: u16,      // identifier
    pub qr: bool,     // 0 for query, 1 for response
    pub opcode: u8,   // 0 for standard query
    pub aa: bool,     // authoritative answer
    pub tc: bool,     // truncated message
    pub rd: bool,     // recursion desired
    pub ra: bool,     // recursion available
    pub z: u8,        // reserved for future use
    pub rcode: u8,    // 0 for no error
    pub qdcount: u16, // number of entries in the question section
    pub ancount: u16, // number of resource records in the answer section
    pub nscount: u16, // number of name server resource records in the authority records section
    pub arcount: u16, // number of resource records in the additional records section
}

impl Header {
    pub(crate) const DNS_HEADER_LEN: usize = 12;

    /// Serialize the DNS header into a byte array
    ///
    /// # Exemple :
    /// ```
    /// use crate::dns::packet::header::Header;
    ///
    /// let header = Header {
    ///     id: 0x1234,
    ///     qr: false,
    ///     opcode: 0,
    ///     aa: false,
    ///     tc: false,
    ///     rd: true,
    ///     ra: false,
    ///     z: 0,
    ///     rcode: 0,
    ///     qdcount: 1,
    ///     ancount: 0,
    ///     nscount: 0,
    ///     arcount: 0,
    /// };
    ///
    /// let bytes = header.to_bytes().unwrap();
    ///
    /// assert_eq!(bytes.len(), Header::DNS_HEADER_LEN);
    /// assert_eq!(bytes[0], 0x12);
    /// assert_eq!(bytes[1], 0x34);
    /// ```
    pub fn to_bytes(&self) -> Result<[u8; Self::DNS_HEADER_LEN], SCloudException> {
        let mut bytes = [0u8; Self::DNS_HEADER_LEN];

        let id_bytes = self.id.to_be_bytes();
        bytes[0] = id_bytes[0];
        bytes[1] = id_bytes[1];

        let mut flags1 = 0u8;
        flags1 |= (self.qr as u8 & 0x1) << 7;
        flags1 |= (self.opcode & 0xF) << 3;
        flags1 |= (self.aa as u8 & 0x1) << 2;
        flags1 |= (self.tc as u8 & 0x1) << 1;
        flags1 |= self.rd as u8 & 0x1;

        let mut flags2 = 0u8;
        flags2 |= (self.ra as u8 & 0x1) << 7;
        flags2 |= (self.z & 0x7) << 4;

        bytes[2] = flags1;
        bytes[3] = flags2;

        let qdcount_bytes = self.qdcount.to_be_bytes();
        bytes[4] = qdcount_bytes[0];
        bytes[5] = qdcount_bytes[1];

        let ancount_bytes = self.ancount.to_be_bytes();
        bytes[6] = ancount_bytes[0];
        bytes[7] = ancount_bytes[1];

        let nscount_bytes = self.nscount.to_be_bytes();
        bytes[8] = nscount_bytes[0];
        bytes[9] = nscount_bytes[1];

        let arcount_bytes = self.arcount.to_be_bytes();
        bytes[10] = arcount_bytes[0];
        bytes[11] = arcount_bytes[1];

        Ok(bytes)
    }

    /// Deserialize the DNS header from a byte array
    ///
    /// # Exemple :
    /// ```
    /// use crate::dns::packet::header::Header;
    ///
    /// let raw_header: [u8; 12] = [
    ///     0x12, 0x34, // ID
    ///     0x01, 0x00, // Flags (standard query, RD = 1)
    ///     0x00, 0x01, // QDCOUNT = 1
    ///     0x00, 0x00, // ANCOUNT = 0
    ///     0x00, 0x00, // NSCOUNT = 0
    ///     0x00, 0x00, // ARCOUNT = 0
    /// ];
    ///
    /// let header = Header::from_bytes(&raw_header).unwrap();
    ///
    /// assert_eq!(header.id, 0x1234);
    /// assert_eq!(header.qr, false);
    /// assert_eq!(header.rd, true);
    /// assert_eq!(header.qdcount, 1);
    /// ```
    pub fn from_bytes(buf: &[u8]) -> Result<Header, SCloudException> {
        if buf.len() == 0 {
            return Err(SCloudException::SCLOUD_HEADER_BYTES_EMPTY);
        }

        if buf.len() < Header::DNS_HEADER_LEN {
            return Err(SCloudException::SCLOUD_HEADER_DESERIALIZATION_FAILED);
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
