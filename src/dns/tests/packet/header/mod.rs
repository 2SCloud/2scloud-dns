
#[cfg(test)]
mod tests {
    use crate::dns::packet::header::Header;
    use crate::exceptions::SCloudException;

    #[test]
    fn test_header_to_bytes() {
        let header: Header = Header{
            id: 43690,
            qr: false,
            opcode: 0,
            aa: false,
            tc: false,
            rd: true,
            ra: false,
            z: 0,
            rcode: 0,
            qdcount: 1,
            ancount: 0,
            nscount: 0,
            arcount: 0,
        };

        let result = Header::to_bytes(&header);

        let expected: &[u8] = &[
            0xAA,0xAA,
            0x01,
            0x00,0x00,0x01,0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00];

        println!("expected: {:?}, got: {:?}", expected, result);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_header_deserialization_failure() {
        let result = Header::from_bytes(&[0xAA, 0xAA, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00]);
        assert!(matches!(result, Err(SCloudException::SCLOUD_HEADER_DESERIALIZATION_FAILED)));
    }

    #[test]
    fn test_header_deserialization_buffer_empty() {
        let result = Header::from_bytes(&[]);
        assert!(matches!(result, Err(SCloudException::SCLOUD_HEADER_BYTES_EMPTY)));
    }

}