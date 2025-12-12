
#[cfg(test)]
mod tests {
    use crate::dns::packet::answer::AnswerSection;
    use crate::dns::records::{DNSClass, DNSRecordType};
    use crate::exceptions::SCloudException;

    #[test]
    fn test_qname_too_long_for_dns() {
        let asec: AnswerSection = AnswerSection{
            q_name: "qnamemorethan63characterstotestifthecodereallypanicornottestdotcom".parse().unwrap(),
            r_type: DNSRecordType::A,
            r_class: DNSClass::IN,
            ttl: 0,
            rdlength: 0,
            rdata: vec![],
        };
        let result = asec.to_bytes();

        println!("expected: {:?}\ngot: {:?}", SCloudException::SCLOUD_ANSWER_DESERIALIZATION_FAILED_LABEL_TOO_LONG, asec.to_bytes().err().unwrap());
        assert_eq!(result.err().unwrap(), SCloudException::SCLOUD_ANSWER_DESERIALIZATION_FAILED_LABEL_TOO_LONG);
    }

    #[test]
    fn test_answer_deserialization_fails_when_buf_lower_than_pos10() {
        let mut buf = vec![3, b'w', b'w', b'w', 0];

        let result = AnswerSection::from_bytes(&buf, 0);

        assert_eq!(
            result.unwrap_err(),
            SCloudException::SCLOUD_ANSWER_DESERIALIZATION_FAILED_BUF_LOWER_THAN_POS10
        );
    }

    #[test]
    fn test_answer_deserialization_fails_when_buf_lower_than_posrd() {
        let mut buf = vec![3, b'w', b'w', b'w', 0];

        buf.extend_from_slice(&[
            0x00, 0x01,
            0x00, 0x01,
            0x00, 0x00, 0x00, 0x10,
            0x00, 0x05,
        ]);

        let result = AnswerSection::from_bytes(&buf, 0);

        assert_eq!(
            result.unwrap_err(),
            SCloudException::SCLOUD_ANSWER_DESERIALIZATION_FAILED_BUF_LOWER_THAN_POSRD
        );
    }
}