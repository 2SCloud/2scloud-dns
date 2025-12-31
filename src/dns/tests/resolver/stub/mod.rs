#[cfg(test)]
mod tests {
    use crate::config::Config;
    use crate::dns::packet::DNSPacket;
    use crate::dns::packet::answer::AnswerSection;
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
        let result =
            StubResolver::new(config.try_get_forwarder_addr_by_name("sta-internal").unwrap())
                .resolve(vec![QuestionSection {
                    q_name: "github.com".to_string(),
                    q_type: DNSRecordType::CNAME,
                    q_class: DNSClass::IN,
                }])
                .unwrap();

        assert!(result.header.qr);
        assert_eq!(result.questions.len(), 1);
        assert_eq!(result.questions[0].q_name, "github.com");

        assert!(
            result.answers
                .as_slice()
                .iter()
                .any(|a| a.r_type == DNSRecordType::A ||
                         a.r_type == DNSRecordType::AAAA ||
                         a.r_type == DNSRecordType::CNAME)
        );

    }

    #[test]
    fn test_force_invalid_id() {
        let mut packet = DNSPacket::new_query(&[]);
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
        let mut packet = DNSPacket::new_query(&[]);
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

    #[test]
    fn resolver_rejects_response_with_mismatching_answer() {
        let question = QuestionSection {
            q_name: "example.com".to_string(),
            q_type: DNSRecordType::A,
            q_class: DNSClass::IN,
        };

        let questions = vec![question.clone()];

        let mut response = DNSPacket {
            header: Header {
                id: 1234,
                qr: true,
                ..Default::default()
            },
            questions: vec![question],
            answers: vec![AnswerSection {
                q_name: "evil.com".to_string(),
                r_type: DNSRecordType::A,
                r_class: DNSClass::IN,
                ttl: 300,
                rdlength: 0,
                rdata: vec![127, 0, 0, 1],
            }],
            authorities: vec![],
            additionals: vec![],
        };

        let result = (|| {
            for question in &questions {
                let mut found = false;

                for answer in &response.answers {
                    if answer.q_name == question.q_name
                        && answer.r_type == question.q_type
                        && answer.r_class == question.q_class
                    {
                        found = true;
                        break;
                    }
                }

                if !found {
                    return Err(SCloudException::SCLOUD_RESOLVER_ANSWER_MISMATCH);
                }
            }

            Ok(response)
        })();

        assert!(matches!(
            result,
            Err(SCloudException::SCLOUD_RESOLVER_ANSWER_MISMATCH)
        ));
    }
}
