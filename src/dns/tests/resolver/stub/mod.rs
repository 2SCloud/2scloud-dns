#[cfg(test)]
mod tests {
    use crate::dns::packet::DNSPacket;
    use crate::dns::packet::authority::AuthoritySection;
    use crate::dns::packet::header::Header;
    use crate::dns::packet::question::QuestionSection;
    use crate::dns::q_class::DNSClass;
    use crate::dns::q_type::DNSRecordType;
    use crate::dns::resolver::stub::StubResolver;
    use std::net::SocketAddr;

    #[test]
    fn test_new_stub_resolver() {
        let result = StubResolver::new("1.1.1.1:53".parse().unwrap());
        let expected = StubResolver {
            server: SocketAddr::new("1.1.1.1".parse().unwrap(), 53),
            timeout: std::time::Duration::from_secs(5),
            retries: 3,
        };

        println!("expected: {:?}\ngot: {:?}", expected, result);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_stub_resolve() {
        let result = StubResolver::new("1.1.1.1:53".parse().unwrap())
            .resolve(vec![QuestionSection {
                q_name: "github.com".to_string(),
                q_type: DNSRecordType::CNAME,
                q_class: DNSClass::IN,
            }])
            .unwrap();

        let expected_packet: DNSPacket = DNSPacket {
            header: Header {
                id: result.header.id,
                qr: true,
                opcode: 0,
                aa: false,
                tc: false,
                rd: true,
                ra: true,
                z: 0,
                rcode: 0,
                qdcount: 1,
                ancount: 0,
                nscount: 1,
                arcount: 0,
            },
            questions: vec![QuestionSection {
                q_name: "github.com".to_string(),
                q_type: DNSRecordType::CNAME,
                q_class: DNSClass::IN,
            }],
            answers: vec![],
            authorities: vec![AuthoritySection {
                q_name: "github.com".to_string(),
                q_type: DNSRecordType::SOA,
                q_class: DNSClass::IN,
                ttl: result.authorities[0].ttl,
                ns_name: "ns-1707.awsdns-21.co.uk".to_string(),
            }],
            additionals: vec![],
        };

        println!("expected: {:?}\ngot: {:?}", expected_packet, result);
        assert_eq!(expected_packet, result)
    }
}
