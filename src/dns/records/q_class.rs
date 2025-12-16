use crate::exceptions::SCloudException;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DNSClass {
    IN,   // Internet
    CS,   // CSNET (historic)
    CH,   // CHAOS
    HS,   // Hesiod
    NONE, // QCLASS NONE (RFC 2136)
    ANY,  // QCLASS ANY  (RFC 1035)
}

impl TryFrom<u16> for DNSClass {
    type Error = SCloudException;

    fn try_from(v: u16) -> Result<DNSClass, Self::Error> {
        match v {
            0 => Ok(DNSClass::NONE),
            1 => Ok(DNSClass::IN),
            2 => Ok(DNSClass::CS),
            3 => Ok(DNSClass::CH),
            4 => Ok(DNSClass::HS),
            255 => Ok(DNSClass::ANY),
            _ => Err(SCloudException::SCLOUD_QCLASS_U16_FOR_DNSCLASS_UNKNOWN),
        }
    }
}

impl TryFrom<DNSClass> for u16 {
    type Error = SCloudException;

    fn try_from(c: DNSClass) -> Result<u16, Self::Error> {
        match c {
            DNSClass::NONE => Ok(0),
            DNSClass::IN => Ok(1),
            DNSClass::CS => Ok(2),
            DNSClass::CH => Ok(3),
            DNSClass::HS => Ok(4),
            DNSClass::ANY => Ok(255),
            _ => Err(SCloudException::SCLOUD_QCLASS_DNSCLASS_FOR_U16_UNKNOWN),
        }
    }
}

impl From<&[u8; 2]> for DNSClass {
    fn from(bytes: &[u8; 2]) -> Self {
        let v = u16::from_be_bytes(*bytes);
        DNSClass::try_from(v).unwrap()
    }
}
