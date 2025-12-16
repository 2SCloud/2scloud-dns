#[cfg(test)]
mod tests {
    use crate::dns::records::DNSClass;

    #[test]
    fn qclass_to_u16() {
        assert_eq!(u16::try_from(DNSClass::NONE).unwrap(), 0);
        assert_eq!(u16::try_from(DNSClass::IN).unwrap(), 1);
        assert_eq!(u16::try_from(DNSClass::CS).unwrap(), 2);
        assert_eq!(u16::try_from(DNSClass::CH).unwrap(), 3);
        assert_eq!(u16::try_from(DNSClass::HS).unwrap(), 4);
        assert_eq!(u16::try_from(DNSClass::ANY).unwrap(), 255);
    }

    #[test]
    fn u16_to_qclass() {
        assert_eq!(DNSClass::try_from(0).unwrap(), DNSClass::NONE);
        assert_eq!(DNSClass::try_from(1).unwrap(), DNSClass::IN);
        assert_eq!(DNSClass::try_from(2).unwrap(), DNSClass::CS);
        assert_eq!(DNSClass::try_from(3).unwrap(), DNSClass::CH);
        assert_eq!(DNSClass::try_from(4).unwrap(), DNSClass::HS);
        assert_eq!(DNSClass::try_from(255).unwrap(), DNSClass::ANY);
    }

    #[test]
    fn test_dnsclass_from_bytes() {
        let bytes = [0x00, 0x01];
        assert_eq!(DNSClass::from(&bytes), DNSClass::IN);

        let bytes = [0x00, 0x03];
        assert_eq!(DNSClass::from(&bytes), DNSClass::CH);

        let bytes = [0x00, 0xFF];
        assert_eq!(DNSClass::from(&bytes), DNSClass::ANY);
    }
}
