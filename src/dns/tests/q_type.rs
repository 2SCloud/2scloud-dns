#[cfg(test)]
mod tests {
    use crate::dns::q_type::DNSRecordType;
    use crate::exceptions::SCloudException;
    use strum::IntoEnumIterator;

    #[test]
    fn test_tryfrom_u16_for_dns_record_type() {
        for rtype in DNSRecordType::iter() {
            if let DNSRecordType::Unknown(_) = rtype {
                continue;
            }

            let code = u16::try_from(rtype).unwrap();
            let back = DNSRecordType::try_from(code).unwrap();

            assert_eq!(back, rtype, "Round-trip failed for {:?}", rtype);
        }
    }

    #[test]
    fn test_tryfrom_u16_for_dns_record_type_unknown() {
        let rt = DNSRecordType::try_from(24616).unwrap();

        assert_eq!(rt, DNSRecordType::Unknown(24616));
    }

    #[test]
    fn test_tryfrom_dns_record_type_for_u16() {
        let result = DNSRecordType::try_from(39).unwrap();
        println!("expected: {:?}\ngot: {:?}", DNSRecordType::DNAME, result);
        assert_eq!(result, DNSRecordType::DNAME);
    }

    #[test]
    fn test_tryfrom_dns_record_type_unknown_to_u16() {
        let result = u16::try_from(DNSRecordType::Unknown(45616)).unwrap();

        assert_eq!(result, 45616);
    }

    #[test]
    fn test_tryfrom_array_for_dns_record_type() {
        let bytes: &[u8; 2] = &[0x00, 0x27];
        let result = DNSRecordType::try_from(bytes).unwrap();
        println!("expected: {:?}\ngot: {:?}", DNSRecordType::DNAME, result);
        assert_eq!(result, DNSRecordType::DNAME);
    }

    #[test]
    fn test_tryfrom_array_for_dns_record_type_unknown() {
        let bytes: &[u8; 2] = &[0x01, 0x80];

        let result = DNSRecordType::try_from(bytes).unwrap();

        assert_eq!(result, DNSRecordType::Unknown(384));
    }
}
