mod header;
mod question;
mod answer;
mod authority;

#[cfg(test)]
mod tests {
    use crate::dns::packet::DNSPacket;
    use crate::dns::packet::header::Header;
    use crate::dns::packet::question::QuestionSection;
    use crate::dns::packet::answer::AnswerSection;
    use crate::dns::packet::authority::AuthoritySection;
    use crate::dns::records::{DNSClass, DNSRecordType};

    #[test]
    fn test_dns_packet_from_bytes() {

        let bytes: &[u8] = &[
            0xAA,0xAA,
            0x01, 0x00,
            0x00, 0x01,
            0x00, 0x01,
            0x00, 0x01,
            0x00,

            // QUESTION SECTION
            0x00, 0x04, 0x72, 0x75, 0x73, 0x74,
            0x06, 0x74, 0x72, 0x65, 0x6E, 0x64, 0x73,
            0x03, 0x63, 0x6F, 0x6D,
            0x00,
            0x00, 0x01,
            0x00, 0x01,

            // ANSWER SECTION
            0x04, 0x72, 0x75, 0x73,
            0x74, 0x06, 0x74, 0x72,
            0x65, 0x6E, 0x64, 0x73,
            0x03, 0x63, 0x6F, 0x6D,
            0x00,
            0x00, 0x01,
            0x00, 0x01,              // CLASS
            0x00, 0x00, 0x00, 0x00,  // TTL
            0x00, 0x00,              // RDLENGTH
            //0x00, // DISABLED: cauz we don't need bytes for `rdata` since `rdlength` == [0x00, 0x00]

            // AUTHORITY SECTION (NS record)
            0x04, 0x72, 0x75, 0x73, 0x74,
            0x06, 0x74, 0x72, 0x65, 0x6E, 0x64, 0x73,
            0x03, 0x63, 0x6F, 0x6D,
            0x00,
            0x00, 0x02,             // TYPE = NS
            0x00, 0x01,             // CLASS = IN
            0x00, 0x00, 0x00, 0x3C, // TTL = 60
            0x00, 0x11,             // RDLENGTH = 17
            0x03, 0x6E, 0x73, 0x31, // ns1
            0x06, 0x74, 0x72, 0x65, 0x6E, 0x64, 0x73, // trends
            0x03, 0x63, 0x6F, 0x6D, // com
            0x00,
        ];


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
                ancount: 1,
                nscount: 1,
                arcount: 0,
            },
            questions: vec![QuestionSection{
                q_name: "rust.trends.com".to_string(),
                q_type: DNSRecordType::A,
                q_class: DNSClass::IN
            }],
            answers: vec![AnswerSection{
                q_name: "rust.trends.com".to_string(),
                r_type: DNSRecordType::A,
                r_class: DNSClass::IN,
                ttl: 0,
                rdlength: 0,
                rdata: vec![]
            }],
            authorities: vec![AuthoritySection{
                q_name: "rust.trends.com".to_string(),
                q_type: DNSRecordType::NS,
                q_class: DNSClass::IN,
                ttl: 60,
                rdlength: 17,
                rdata: vec![0x03, 0x6E, 0x73, 0x31,
                            0x06, 0x74, 0x72, 0x65, 0x6E, 0x64, 0x73,
                            0x03, 0x63, 0x6F, 0x6D,
                            0x00
                ]
            }],
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
                ancount: 1,
                nscount: 0,
                arcount: 0,
            },
            questions: vec![QuestionSection{
                q_name: "rust.trends.com".to_string(),
                q_type: DNSRecordType::A,
                q_class: DNSClass::IN,
            }],
            answers: vec![AnswerSection{
                q_name: "rust.trends.com".to_string(),
                r_type: DNSRecordType::A,
                r_class: DNSClass::IN,
                ttl: 0,
                rdlength: 0,
                rdata: vec![],
            }],
            authorities: vec![],
            additionals: vec![],
        };

        let expected_bytes: &[u8] = &[0xAA,0xAA,
            0x01, 0x00,
            0x00, 0x01,
            0x00, 0x01,
            0x00,
            0x00,
            0x00,
            0x00, 0x04, 0x72, 0x75, 0x73, 0x74,
            0x06, 0x74, 0x72, 0x65, 0x6E, 0x64, 0x73,
            0x03, 0x63, 0x6F, 0x6D,
            0x00,
            0x00, 0x01,
            0x00, 0x01,
            0x04, 0x72, 0x75, 0x73,
            0x74, 0x06, 0x74, 0x72,
            0x65, 0x6E, 0x64, 0x73,
            0x03, 0x63, 0x6F, 0x6D,
            0x00, 0x00, 0x01, 0x00,
            0x01, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00
        ];

        let result = DNSPacket::to_bytes(packet).unwrap();

        println!("expected: {:?}\ngot: {:?}", expected_bytes, result);
        assert_eq!(expected_bytes, result);
    }

}