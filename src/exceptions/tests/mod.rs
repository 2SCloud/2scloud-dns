#[cfg(test)]
mod tests {
    use crate::exceptions::SCloudException;
    use strum::IntoEnumIterator;

    #[test]
    fn test_exceptions_to_str() {
        let ex_msg_array: [&'static str; 33] = [
            // HEADER SECTION
            "Buffer length is less than header length.",
            "The header is empty.",
            // QUESTION SECTION
            "Buffer length is less than question section length.",
            "`q_name` too long.",
            // ANSWER SECTION
            "Buffer length is less than answer section length.",
            "Label too long for DNS.",
            "Failed to parse DNS answer section: not enough bytes for TYPE, CLASS, TTL, and RDLENGTH.",
            "Failed to parse DNS answer section: RDLENGTH exceeds remaining buffer size.",
            // AUTHORITY SECTION
            "Buffer length is less than authority section length.",
            "Impossible to deserialize, `buf.len()` is lower than `pos+10`. (buf too short)",
            // ADDITIONAL SECTION
            "Buffer length is less than additional section length.",
            "Impossible to deserialize, `buf.len()` is lower than `pos+10`. (buf too short)",
            "`q_name` too long.",
            "Buffer length is less than authority section length.",
            // QNAME
            "Impossible to parse the `q_name`, check if a `q_name` is provided.",
            "Impossible to parse the `q_name`, pos is greater than buffer length.",
            "Impossible to parse the `q_name`, pos and len are greater than buffer length.",
            "Impossible to parse the `q_name`, compression 0xC0xx failed.",
            // QTYPE
            "Unknown `q_type`.",
            // QCLASS
            "Unknown `q_class`.",
            "Unknown `q_class`.",
            // STUB RESOLVER
            "[STUB_RESOLVER] Invalid DNS ID (difference between `response.header.id` and `request_id`).",
            "[STUB_RESOLVER] Invalid DNS response.",
            "[STUB_RESOLVER] Failed to create UDP socket.",
            "[STUB_RESOLVER] Failed to read, socket timeout.",
            "[STUB_RESOLVER] Failed to send to socket.",
            "[STUB_RESOLVER] Failed to receive from socket.",
            // ZONES
            "[ZONE_PARSER] Zone file not found.",
            "[ZONE_PARSER] Zone file is empty.",
            "[ZONE_PARSER] `zone_parser()` failed to read the zone file.",
            "[ZONE_PARSER] `zone_parser()` detect TTL field but failed to read this field.",
            // CONFIG
            "[SCLOUD_CONFIG] Configuration file not found.",
            "[SCLOUD_CONFIG] Error while parsing the JSON file.",
        ];

        let mut i = 0;
        for ex in SCloudException::iter() {
            let msg = ex.to_str();
            println!("variant: {:?}, msg: {}", ex, msg);
            assert_eq!(msg, ex_msg_array[i], "Mismatch for variant {:?}", ex);
            i += 1;
        }

        assert_eq!(i, ex_msg_array.len(), "Number of exceptions mismatch");
    }

    #[test]
    fn test_exceptions_debug() {
        for ex in SCloudException::iter() {
            let debug_str = format!("{:?}", ex);
            assert!(!debug_str.is_empty(), "Debug string should not be empty");
        }
    }

    #[test]
    fn test_exceptions_partial_eq() {
        for ex in SCloudException::iter() {
            assert_eq!(ex, ex.clone());
        }
    }

    #[test]
    fn test_exceptions_iter_count() {
        let count = SCloudException::iter().count();
        let expected_count = 31;
        assert_eq!(count, expected_count);
    }

    #[test]
    fn test_all_exceptions_covered() {
        for ex in SCloudException::iter() {
            let msg = ex.to_str();
            assert!(
                !msg.is_empty(),
                "to_str() returned empty string for variant {:?}",
                ex
            );
        }
    }
}
