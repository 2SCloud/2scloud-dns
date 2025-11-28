#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DNSClass {
    IN,   // Internet
    CS,   // CSNET (historic)
    CH,   // CHAOS
    HS,   // Hesiod
    NONE, // QCLASS NONE (RFC 2136)
    ANY,  // QCLASS ANY  (RFC 1035)
    Unknown(u16),
}

impl From<u16> for DNSClass {
    fn from(v: u16) -> Self {
        match v {
            0 => DNSClass::NONE,
            1 => DNSClass::IN,
            2 => DNSClass::CS,
            3 => DNSClass::CH,
            4 => DNSClass::HS,
            255 => DNSClass::ANY,
            other => DNSClass::Unknown(other),
        }
    }
}

impl From<DNSClass> for u16 {
    fn from(c: DNSClass) -> u16 {
        match c {
            DNSClass::NONE => 0,
            DNSClass::IN => 1,
            DNSClass::CS => 2,
            DNSClass::CH => 3,
            DNSClass::HS => 4,
            DNSClass::ANY => 255,
            DNSClass::Unknown(v) => v,
        }
    }
}

impl From<&[u8; 2]> for DNSClass {
    fn from(bytes: &[u8; 2]) -> Self {
        let v = u16::from_be_bytes(*bytes);
        DNSClass::from(v)
    }
}
