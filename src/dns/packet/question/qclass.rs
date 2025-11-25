#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DNSClass {
    IN,
    CS,
    CH,
    HS,
    NONE,
    ANY,
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
