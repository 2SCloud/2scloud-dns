use crate::exceptions::SCloudException;

/// DNS CLASS field (QCLASS / CLASS) as defined in DNS RFCs.
///
/// This enum represents the DNS class of a resource record or query.
/// The most common class is `IN` (Internet).
///
/// Supported classes:
/// - IN   (1): Internet
/// - CS   (2): CSNET (obsolete)
/// - CH   (3): CHAOS
/// - HS   (4): Hesiod
/// - NONE (0): Used in dynamic update (RFC 2136)
/// - ANY  (255): Wildcard class (RFC 1035)
///
/// The enum supports conversion to and from the on-the-wire `u16`
/// representation used in DNS packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DNSClass {
    IN,
    CS,
    CH,
    HS,
    NONE,
    ANY,
}

impl TryFrom<u16> for DNSClass {
    type Error = SCloudException;

    /// Convert a raw DNS CLASS value (`u16`) into a `DNSClass`.
    ///
    /// # Errors
    /// Returns `SCLOUD_QCLASS_U16_FOR_DNSCLASS_UNKNOWN` if the value
    /// does not match any known DNS class.
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

    /// Convert a `DNSClass` into its DNS wire format (`u16`).
    ///
    /// # Errors
    /// This error should never occur unless an invalid enum variant
    /// is introduced.
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
    /// Convert a 2-byte DNS wire representation into a `DNSClass`.
    ///
    /// # Panics
    /// Panics if the value does not correspond to a known DNS class.
    fn from(bytes: &[u8; 2]) -> Self {
        let v = u16::from_be_bytes(*bytes);
        DNSClass::try_from(v).unwrap()
    }
}
