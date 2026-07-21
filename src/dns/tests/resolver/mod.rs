mod stub;

#[cfg(test)]
mod tests {
    use crate::dns::packet::DNSPacket;
    use crate::dns::packet::additional::AdditionalSection;
    use crate::dns::packet::answer::AnswerSection;
    use crate::dns::packet::authority::AuthoritySection;
    use crate::dns::packet::header::Header;
    use crate::dns::packet::question::QuestionSection;
    use crate::{dns, exceptions};

    #[test]
    fn test_check_response_diff() {
        let origin_q = QuestionSection {
            q_name: "github.com".to_string(),
            q_type: dns::q_type::DNSRecordType::A,
            q_class: dns::q_class::DNSClass::IN,
        };
        let response_packet = DNSPacket {
            header: Header::default(),
            questions: vec![QuestionSection {
                q_name: "github.com".to_string(),
                q_type: dns::q_type::DNSRecordType::A,
                q_class: dns::q_class::DNSClass::IN,
            }],
            answers: vec![AnswerSection {
                q_name: "github.com".to_string(),
                r_type: dns::q_type::DNSRecordType::A,
                r_class: dns::q_class::DNSClass::IN,
                ttl: 0,
                rdlength: 0,
                rdata: vec![],
            }],
            authorities: vec![],
            additionals: vec![],
        };
        dns::resolver::check_response_diff(response_packet, &[origin_q]).unwrap();
        assert_eq!((), ());
    }

    #[test]
    fn test_check_response_diff_err() {
        let origin_q = QuestionSection {
            q_name: "github.com".to_string(),
            q_type: dns::q_type::DNSRecordType::A,
            q_class: dns::q_class::DNSClass::IN,
        };
        let response_packet = DNSPacket {
            header: Header::default(),
            questions: vec![QuestionSection {
                q_name: "github.com".to_string(),
                q_type: dns::q_type::DNSRecordType::A,
                q_class: dns::q_class::DNSClass::IN,
            }],
            answers: vec![AnswerSection {
                q_name: "apple.com".to_string(),
                r_type: dns::q_type::DNSRecordType::A,
                r_class: dns::q_class::DNSClass::IN,
                ttl: 0,
                rdlength: 0,
                rdata: vec![],
            }],
            authorities: vec![],
            additionals: vec![],
        };
        let result = dns::resolver::check_response_diff(response_packet, &[origin_q]).unwrap_err();
        assert_eq!(
            result,
            exceptions::SCloudException::SCLOUD_RESOLVER_RESPONSE_MISMATCH
        );
    }

    #[test]
    fn test_check_answer_diff() {
        let q = QuestionSection {
            q_name: "github.com".to_string(),
            q_type: dns::q_type::DNSRecordType::A,
            q_class: dns::q_class::DNSClass::IN,
        };
        let a = AnswerSection {
            q_name: "github.com".to_string(),
            r_type: dns::q_type::DNSRecordType::A,
            r_class: dns::q_class::DNSClass::IN,
            ttl: 0,
            rdlength: 0,
            rdata: vec![],
        };
        dns::resolver::check_answer_diff(&[q], &[a]).unwrap();
        assert_eq!((), ());
    }

    #[test]
    fn test_check_answer_diff_err() {
        let q = QuestionSection {
            q_name: "github.com".to_string(),
            q_type: dns::q_type::DNSRecordType::A,
            q_class: dns::q_class::DNSClass::IN,
        };
        let a = AnswerSection {
            q_name: "apple.com".to_string(),
            r_type: dns::q_type::DNSRecordType::A,
            r_class: dns::q_class::DNSClass::IN,
            ttl: 0,
            rdlength: 0,
            rdata: vec![],
        };
        let result = dns::resolver::check_answer_diff(&[q], &[a]).unwrap_err();
        assert_eq!(
            result,
            exceptions::SCloudException::SCLOUD_RESOLVER_ANSWER_QNAME_MISMATCH
        );
    }

    #[test]
    fn test_check_authority_diff() {
        let q = QuestionSection {
            q_name: "github.com".to_string(),
            q_type: dns::q_type::DNSRecordType::A,
            q_class: dns::q_class::DNSClass::IN,
        };
        let a = AuthoritySection {
            q_name: "github.com".to_string(),
            q_type: dns::q_type::DNSRecordType::A,
            q_class: dns::q_class::DNSClass::IN,
            ttl: 0,
            ns_name: "".to_string(),
        };
        dns::resolver::check_authority_diff(&[q], &[a]).unwrap();
        assert_eq!((), ());
    }

    #[test]
    fn test_check_authority_diff_err() {
        let q = QuestionSection {
            q_name: "github.com".to_string(),
            q_type: dns::q_type::DNSRecordType::A,
            q_class: dns::q_class::DNSClass::IN,
        };
        let a = AuthoritySection {
            q_name: "apple.com".to_string(),
            q_type: dns::q_type::DNSRecordType::A,
            q_class: dns::q_class::DNSClass::IN,
            ttl: 0,
            ns_name: "".to_string(),
        };
        let result = dns::resolver::check_authority_diff(&[q], &[a]).unwrap_err();
        assert_eq!(
            result,
            exceptions::SCloudException::SCLOUD_RESOLVER_AUTHORITY_QNAME_MISMATCH
        );
    }

    #[test]
    fn test_check_additionnal_diff() {
        let q = QuestionSection {
            q_name: "github.com".to_string(),
            q_type: dns::q_type::DNSRecordType::A,
            q_class: dns::q_class::DNSClass::IN,
        };
        let a = AdditionalSection {
            q_name: "github.com".to_string(),
            q_type: dns::q_type::DNSRecordType::A,
            q_class: dns::q_class::DNSClass::IN,
            ttl: 0,
            rdlength: 0,
            rdata: vec![],
        };
        dns::resolver::check_additional_diff(&[q], &[a]).unwrap();
        assert_eq!((), ());
    }

    #[test]
    fn test_check_additional_diff_err() {
        let q = QuestionSection {
            q_name: "github.com".to_string(),
            q_type: dns::q_type::DNSRecordType::A,
            q_class: dns::q_class::DNSClass::IN,
        };
        let a = AdditionalSection {
            q_name: "apple.com".to_string(),
            q_type: dns::q_type::DNSRecordType::A,
            q_class: dns::q_class::DNSClass::IN,
            ttl: 0,
            rdlength: 0,
            rdata: vec![],
        };
        let result = dns::resolver::check_additional_diff(&[q], &[a]).unwrap_err();
        assert_eq!(
            result,
            exceptions::SCloudException::SCLOUD_RESOLVER_ADDITIONNAL_QNAME_MISMATCH
        );
    }
}
