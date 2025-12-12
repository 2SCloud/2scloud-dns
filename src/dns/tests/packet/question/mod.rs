
#[cfg(test)]
mod tests {
    use crate::dns::packet::question::QuestionSection;
    use crate::dns::records::{DNSClass, DNSRecordType};
    use crate::exceptions::SCloudException;

    #[test]
    fn test_qname_too_long_for_dns() {
        let qs: QuestionSection = QuestionSection{
            q_name: "qnamemorethan63characterstotestifthecodereallypanicornottestdotcom".parse().unwrap(),
            q_type: DNSRecordType::A,
            q_class: DNSClass::IN,
        };
        let result = qs.to_bytes();

        println!("expected: {:?}\ngot: {:?}", SCloudException::SCLOUD_QUESTION_SERIALIZATION_FAILED_QNAME_TOO_LONG, result);
    }

}