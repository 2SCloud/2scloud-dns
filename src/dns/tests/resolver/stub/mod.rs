#[cfg(test)]
mod tests {
    use crate::config::Config;
    use crate::dns::packet::DNSPacket;
    use crate::dns::packet::header::Header;
    use crate::dns::packet::question::QuestionSection;
    use crate::dns::q_class::DNSClass;
    use crate::dns::q_type::DNSRecordType;
    use crate::dns::resolver::stub::StubResolver;
    use crate::exceptions::SCloudException;
    use std::net::SocketAddr;
    use std::path::Path;

    pub fn resolve_with_fake(
        stub: StubResolver,
        questions: Vec<QuestionSection>,
        fake_response: Option<DNSPacket>,
    ) -> Result<DNSPacket, SCloudException> {
        if let Some(resp) = fake_response {
            let request_id = 42;
            if resp.header.id != request_id {
                return Err(SCloudException::SCLOUD_STUB_RESOLVER_INVALID_DNS_ID);
            }
            if !resp.header.qr {
                return Err(SCloudException::SCLOUD_STUB_RESOLVER_INVALID_DNS_RESPONSE);
            }
            return Ok(resp);
        }

        stub.resolve(questions)
    }

    #[test]
    fn test_new_stub_resolver() {
        let config = Config::from_file(Path::new("./config/config.json")).unwrap();
        let result = StubResolver::new("1.1.1.1:53".parse().unwrap());
        let expected = StubResolver {
            server: SocketAddr::new("1.1.1.1".parse().unwrap(), 53),
            timeout: std::time::Duration::from_secs(config.server.graceful_shutdown_timeout_secs),
            retries: 3,
        };

        println!("expected: {:?}\ngot: {:?}", expected, result);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_stub_resolve() {
        let config = Config::from_file(Path::new("./config/config.json")).unwrap();
        let result = StubResolver::new(
            config
                .try_get_forwarder_addr_by_name("cloudflare")
                .unwrap(),
        ).resolve(vec![QuestionSection {
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
            authorities: vec![],
            additionals: vec![],
        };

        assert_eq!(expected_packet.header, result.header);
        assert_eq!(expected_packet.questions, result.questions);
        assert_eq!(expected_packet.answers, result.answers);
        assert_eq!(expected_packet.additionals, result.additionals);

        assert_eq!(result.authorities.len(), 1);
        let auth = &result.authorities[0];

        assert_eq!(auth.q_name, "github.com");
        assert_eq!(auth.q_type, DNSRecordType::SOA);
        assert_eq!(auth.q_class, DNSClass::IN);
        assert!(auth.ttl > 0);
        assert!(!auth.ns_name.is_empty());
    }

    #[test]
    fn test_force_invalid_id() {
        let mut packet = DNSPacket::new_query(vec![]);
        let request_id = packet.header.id;

        let invalid_id_packet = DNSPacket {
            header: Header {
                id: request_id + 1,
                qr: true,
                ..packet.header
            },
            ..packet
        };

        let result = resolve_with_fake(
            StubResolver::new("1.1.1.1:53".parse().unwrap()),
            vec![],
            Some(invalid_id_packet),
        )
        .err()
        .unwrap();

        println!(
            "expected: {:?}\ngot: {:?}",
            SCloudException::SCLOUD_STUB_RESOLVER_INVALID_DNS_ID,
            result
        );
        assert_eq!(result, SCloudException::SCLOUD_STUB_RESOLVER_INVALID_DNS_ID);
    }

    #[test]
    fn test_force_invalid_response() {
        let mut packet = DNSPacket::new_query(vec![]);
        let request_id = 42;

        let invalid_qr_packet = DNSPacket {
            header: Header {
                id: request_id,
                qr: false,
                ..packet.header
            },
            ..packet
        };

        let result = resolve_with_fake(
            StubResolver::new("192.0.0.245:53".parse().unwrap()),
            vec![],
            Some(invalid_qr_packet),
        )
        .err()
        .unwrap();

        println!(
            "expected: {:?}\ngot: {:?}",
            SCloudException::SCLOUD_STUB_RESOLVER_INVALID_DNS_RESPONSE,
            result
        );
        assert_eq!(
            result,
            SCloudException::SCLOUD_STUB_RESOLVER_INVALID_DNS_RESPONSE
        );
    }
}
