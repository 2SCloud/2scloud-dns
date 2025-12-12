mod header;
mod question;

#[cfg(test)]
mod tests {
    use crate::dns::packet::DNSPacket;
    use crate::dns::packet::header::Header;
    use crate::dns::packet::question::QuestionSection;
    use crate::dns::records::{DNSClass, DNSRecordType};

    #[test]
    fn test_dns_packet_from_bytes() {

        let bytes: &[u8] = &[
            0xAA,0xAA,
            0x01,
            0x00,0x00,0x01,0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x04,0x72,0x75,0x73,0x74,
            0x06,0x74,0x72,0x65,0x6E,0x64,0x73,
            0x03,0x63,0x6F,0x6D,
            0x00,
            0x00,0x01,
            0x00,0x01];

        let expected_packet = DNSPacket{
            header: Header {
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
            },
            questions: vec![QuestionSection{
                q_name: "rust.trends.com".to_string(),
                q_type: DNSRecordType::A,
                q_class: DNSClass::IN,
            }],
            answers: vec![],
            authorities: vec![],
            additionals: vec![],
        };

        let result = DNSPacket::from_bytes(bytes).unwrap();
        println!("expected: {:?}, got: {:?}", expected_packet, result);
        assert_eq!(expected_packet, result)

    }

    #[test]
    fn test_dns_packet_to_bytes() {
        let packet: DNSPacket = DNSPacket{
            header: Header {
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
            },
            questions: vec![QuestionSection{
                q_name: "rust.trends.com".to_string(),
                q_type: DNSRecordType::A,
                q_class: DNSClass::IN,
            }],
            answers: vec![],
            authorities: vec![],
            additionals: vec![],
        };

        let expected_bytes: &[u8] = &[0xAA,0xAA,
            0x01,
            0x00,0x00,0x01,0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00, 0x04, 0x72, 0x75, 0x73, 0x74,
            0x06, 0x74, 0x72, 0x65, 0x6E, 0x64, 0x73,
            0x03, 0x63, 0x6F, 0x6D,
            0x00,
            0x00, 0x01,
            0x00, 0x01
        ];

        let result = DNSPacket::to_bytes(packet).unwrap();

        println!("expected: {:?}\ngot: {:?}", expected_bytes, result);
        assert_eq!(expected_bytes, result);
    }

}