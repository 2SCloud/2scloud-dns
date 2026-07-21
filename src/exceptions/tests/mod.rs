#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use crate::exceptions::SCloudException;
    use strum::IntoEnumIterator;

    fn cases() -> Vec<(u16, SCloudException)> {
        vec![
            (0, SCloudException::SCLOUD_HEADER_DESERIALIZATION_FAILED),
            (1, SCloudException::SCLOUD_HEADER_BYTES_EMPTY),
            (2, SCloudException::SCLOUD_QUESTION_DESERIALIZATION_FAILED),
            (
                3,
                SCloudException::SCLOUD_QUESTION_SERIALIZATION_FAILED_QNAME_TOO_LONG,
            ),
            (4, SCloudException::SCLOUD_ANSWER_DESERIALIZATION_FAILED),
            (
                5,
                SCloudException::SCLOUD_ANSWER_DESERIALIZATION_FAILED_LABEL_TOO_LONG,
            ),
            (
                6,
                SCloudException::SCLOUD_IMPOSSIBLE_PARSE_ANSWER_HEADER_TOO_SHORT,
            ),
            (
                7,
                SCloudException::SCLOUD_IMPOSSIBLE_PARSE_ANSWER_RDATA_OUT_OF_BOUNDS,
            ),
            (
                8,
                SCloudException::SCLOUD_AUTHORITY_DESERIALIZATION_FAILED_RDATA_OUT_OF_BOUNDS,
            ),
            (
                9,
                SCloudException::SCLOUD_AUTHORITY_DESERIALIZATION_FAILED_BUF_TOO_SHORT,
            ),
            (
                10,
                SCloudException::SCLOUD_ADDITIONAL_DESERIALIZATION_FAILED,
            ),
            (
                11,
                SCloudException::SCLOUD_ADDITIONAL_DESERIALIZATION_FAILED_BUF_TOO_SHORT,
            ),
            (
                12,
                SCloudException::SCLOUD_ADDITIONAL_DESERIALIZATION_FAILED_QNAME_TOO_LONG,
            ),
            (
                13,
                SCloudException::SCLOUD_ADDITIONAL_DESERIALIZATION_FAILED_RDATA_OUT_OF_BOUNDS,
            ),
            (14, SCloudException::SCLOUD_IMPOSSIBLE_PARSE_QNAME),
            (
                15,
                SCloudException::SCLOUD_IMPOSSIBLE_PARSE_QNAME_POS_GREATER_THAN_BUF,
            ),
            (
                16,
                SCloudException::SCLOUD_IMPOSSIBLE_PARSE_QNAME_POS_AND_LEN_GREATER_THAN_BUF,
            ),
            (
                17,
                SCloudException::SCLOUD_IMPOSSIBLE_PARSE_QNAME_COMPRESSION_FAILED,
            ),
            (
                18,
                SCloudException::SCLOUD_QTYPE_U16_FOR_DNSRECORDTYPE_UNKNOWN,
            ),
            (19, SCloudException::SCLOUD_QCLASS_U16_FOR_DNSCLASS_UNKNOWN),
            (20, SCloudException::SCLOUD_QCLASS_DNSCLASS_FOR_U16_UNKNOWN),
            (21, SCloudException::SCLOUD_STUB_RESOLVER_INVALID_DNS_ID),
            (
                22,
                SCloudException::SCLOUD_STUB_RESOLVER_INVALID_DNS_RESPONSE,
            ),
            (
                23,
                SCloudException::SCLOUD_STUB_RESOLVER_FAILED_TO_CREATE_SOCKET,
            ),
            (
                24,
                SCloudException::SCLOUD_STUB_RESOLVER_FAILED_TO_READ_SOCKET_TIMEOUT,
            ),
            (
                25,
                SCloudException::SCLOUD_STUB_RESOLVER_FAILED_TO_SEND_TO_SOCKET,
            ),
            (
                26,
                SCloudException::SCLOUD_STUB_RESOLVER_FAILED_TO_RECV_FROM_SOCKET,
            ),
            (27, SCloudException::SCLOUD_RESOLVER_RESPONSE_MISMATCH),
            (28, SCloudException::SCLOUD_RESOLVER_RECORD_OUT_OF_ZONE),
            (29, SCloudException::SCLOUD_RESOLVER_ANSWER_QNAME_MISMATCH),
            (
                30,
                SCloudException::SCLOUD_RESOLVER_AUTHORITY_QNAME_MISMATCH,
            ),
            (
                31,
                SCloudException::SCLOUD_RESOLVER_ADDITIONNAL_QNAME_MISMATCH,
            ),
            (32, SCloudException::SCLOUD_ZONE_PARSER_FILE_NOT_FOUND),
            (33, SCloudException::SCLOUD_ZONE_PARSER_FILE_EMPTY),
            (
                34,
                SCloudException::SCLOUD_ZONE_PARSER_FAILED_TO_READ_ZONE_FILE,
            ),
            (
                35,
                SCloudException::SCLOUD_ZONE_PARSER_FAILED_TO_READ_TTL_FIELD,
            ),
            (36, SCloudException::SCLOUD_CONFIG_FILE_NOT_FOUND),
            (37, SCloudException::SCLOUD_CONFIG_IMPOSSIBLE_TO_PARSE_JSON),
            (38, SCloudException::SCLOUD_CONFIG_MISSING_FORWARDER),
            (39, SCloudException::SCLOUD_CONFIG_MISSING_ADDRESS),
            (40, SCloudException::SCLOUD_CONFIG_IMPOSSIBLE_TO_PARSE_ADDR),
            (41, SCloudException::SCLOUD_CONFIG_INVALID_SERVER_PORT),
            (42, SCloudException::SCLOUD_CONFIG_INVALID_MAX_UDP_PAYLOAD),
            (43, SCloudException::SCLOUD_CONFIG_INVALID_DNS_LIMITS),
            (44, SCloudException::SCLOUD_CONFIG_INVALID_LISTENER),
            (45, SCloudException::SCLOUD_CONFIG_DUPLICATE_LISTENER_NAME),
            (46, SCloudException::SCLOUD_CONFIG_INVALID_LISTENER_PORT),
            (
                47,
                SCloudException::SCLOUD_CONFIG_INVALID_LISTENER_PROTOCOLS,
            ),
            (48, SCloudException::SCLOUD_CONFIG_TLS_MISSING_CERT),
            (49, SCloudException::SCLOUD_CONFIG_TLS_MISSING_KEY),
            (50, SCloudException::SCLOUD_CONFIG_TLS_REQUIRES_TCP),
            (51, SCloudException::SCLOUD_CONFIG_INVALID_DOH),
            (52, SCloudException::SCLOUD_CONFIG_UNKNOWN_ACL_REFERENCE),
            (53, SCloudException::SCLOUD_CONFIG_INVALID_FORWARDER),
            (54, SCloudException::SCLOUD_CONFIG_DUPLICATE_FORWARDER_NAME),
            (55, SCloudException::SCLOUD_CONFIG_INVALID_ZONE),
            (56, SCloudException::SCLOUD_CONFIG_DUPLICATE_ZONE_NAME),
            (57, SCloudException::SCLOUD_CONFIG_ZONE_MISSING_FILE),
            (58, SCloudException::SCLOUD_CONFIG_SLAVE_MISSING_MASTERS),
            (
                59,
                SCloudException::SCLOUD_CONFIG_FORWARD_ZONE_MISSING_FORWARDERS,
            ),
            (60, SCloudException::SCLOUD_CONFIG_INVALID_INLINE_ZONE),
            (61, SCloudException::SCLOUD_CONFIG_UNKNOWN_TSIG_KEY),
            (62, SCloudException::SCLOUD_CONFIG_MX_MISSING_PRIORITY),
            (63, SCloudException::SCLOUD_CONFIG_PRIORITY_ON_NON_MX),
            (64, SCloudException::SCLOUD_CONFIG_INVALID_VIEW),
            (65, SCloudException::SCLOUD_CONFIG_DUPLICATE_VIEW_NAME),
            (66, SCloudException::SCLOUD_CONFIG_INVALID_DYNUPDATE),
            (67, SCloudException::SCLOUD_CONFIG_DYNUPDATE_UNKNOWN_ZONE),
            (68, SCloudException::SCLOUD_LOGGING_PATH_CREATION_FAILED),
            (
                69,
                SCloudException::SCLOUD_LOGGING_FILE_CREATION_OR_OPENING_FAILED,
            ),
            (70, SCloudException::SCLOUD_WORKER_FAILED_TO_SPAWN),
            (71, SCloudException::SCLOUD_WORKER_TX_NOT_SET),
            (72, SCloudException::SCLOUD_WORKER_RX_NOT_SET),
            (73, SCloudException::SCLOUD_WORKER_LISTENER_BIND_FAILED),
            (74, SCloudException::SCLOUD_WORKER_FAILED_TO_CREATE_DECODER),
            (75, SCloudException::SCLOUD_WORKER_UNKNOWN_TYPE),
            (76, SCloudException::SCLOUD_WORKER_LISTENER_RECV_FAILED),
            (
                77,
                SCloudException::SCLOUD_QTYPE_DNSRECORDTYPE_FOR_U16_UNKNOWN,
            ),
        ]
    }

    #[test]
    fn test_exceptions_to_str() {
        let ex_msg_array: [&'static str; 83] = [
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
            "Unknown `q_type`, failed to find a DNSRecordType for a u16.",
            "Unknown `q_type`, failed to find a u16 for a DNSRecordType.",
            // QCLASS
            "Unknown `q_class`, failed to find a DNSClass for a u16.",
            "Unknown `q_class`, failed to find a u16 for a DNSClass.",
            // STUB RESOLVER
            "Invalid DNS ID (difference between `response.header.id` and `request_id`).",
            "Invalid DNS response.",
            "Failed to create UDP socket.",
            "Failed to read, socket timeout.",
            "Failed to send to socket.",
            "Failed to receive from socket.",
            // RESOLVER
            "DNS response failed validation against the original query.",
            "Record out of zone.",
            "`AnswerSection.q_name` is not the same as `QuestionSection.q_name`",
            "`AuthoritySection.q_name` is not the same as `QuestionSection.q_name`",
            "`AdditionnalSection.q_name` is not the same as `QuestionSection.q_name`",
            // ZONES
            "Zone file not found.",
            "Zone file is empty.",
            "`zone_parser()` failed to read the zone file.",
            "`zone_parser()` detect TTL field but failed to read this field.",
            // CONFIG
            "Configuration file not found.",
            "Error while parsing the JSON file.",
            "Missing forwarder.",
            "Missing address.",
            "Error while parsing the IP address.",
            "Invalid server port (must be between 1 and 65535).",
            "Invalid max UDP payload size.",
            "Invalid DNS limits (label length, domain length, or packet size).",
            "Invalid listener configuration.",
            "Duplicate listener name detected.",
            "Invalid listener port.",
            "Listener has no valid protocol defined.",
            "TLS enabled but certificate path is missing.",
            "TLS enabled but private key path is missing.",
            "TLS listeners require TCP support.",
            "Invalid DNS-over-HTTPS (DoH) configuration.",
            "Unknown or invalid ACL reference.",
            "Invalid forwarder configuration.",
            "Duplicate forwarder name detected.",
            "Invalid DNS zone configuration.",
            "Duplicate zone name detected.",
            "Zone file path is missing.",
            "Slave zone has no master servers defined.",
            "Forward zone has no forwarders defined.",
            "Inline zone is invalid (missing records or SOA).",
            "Referenced TSIG key does not exist.",
            "MX record is missing priority field.",
            "Priority field is only allowed on MX records.",
            "Invalid DNS view configuration.",
            "Duplicate view name detected.",
            "Invalid dynamic update configuration.",
            "Dynamic update references an unknown zone.",
            // LOGGING
            "Logging path creation failed.",
            "Log file creation/opening failed.",
            // WORKER
            "Failed to link the worker to the thread, and cannot spawn a worker.",
            "`dns_tx` is not set for this worker.",
            "`dns_rx` is not set for this worker.",
            "Listener bind just failed at 'threads::run(&self)'.",
            "Failed to create a decoding worker.",
            "Unknown worker type.",
            "Failed to .acquire_owned(), because the Semaphore is closed.",
            // TCP ACCEPTOR
            "Impossible to create a TCP_ACCEPTOR worker, socket creation failed.",
            "TCP_ACCEPTOR recv() failed.",
            "Impossible to bind TCP_ACCEPTOR socket, most probable cause: another worker is already using this port.",
            // LISTENER
            "Listener recv() failed.",
            "LISTENER worker spawned directly — use TCP_ACCEPTOR instead.",
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
        let expected_count = 83;
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

    #[test]
    fn tryfrom_u16_to_exception_all_mappings_ok() {
        for (code, expected) in cases() {
            let got = SCloudException::try_from(code).unwrap_or_else(|e| {
                panic!("code {code}: expected Ok({expected:?}) got Err({e:?})")
            });
            assert_eq!(got, expected, "code {code}: mauvais mapping");
        }
    }

    #[test]
    fn tryfrom_u16_to_exception_out_of_range_is_err() {
        for &code in &[83u16, 100, 1000, u16::MAX] {
            let err = SCloudException::try_from(code)
                .expect_err(&format!("code {code}: expected Err, got Ok"));
            assert_eq!(
                err,
                SCloudException::SCLOUD_WORKER_UNKNOWN_TYPE,
                "code {code}: erreur attendue différente"
            );
        }
    }

    #[test]
    fn tryfrom_exception_to_u16_all_mappings_ok() {
        for (expected_code, exc) in cases() {
            let got_code = u16::try_from(exc.clone()).unwrap_or_else(|e| {
                panic!("exc {exc:?}: expected Ok({expected_code}) got Err({e:?})")
            });
            assert_eq!(got_code, expected_code, "exc {exc:?}: mauvais mapping");
        }
    }

    #[test]
    fn round_trip_u16_to_exception_to_u16() {
        for (code, _) in cases() {
            let exc = SCloudException::try_from(code).unwrap();
            let code2 = u16::try_from(exc).unwrap();
            assert_eq!(code2, code, "round-trip cassé pour code {code}");
        }
    }

    #[test]
    fn round_trip_exception_to_u16_to_exception() {
        for (_, exc) in cases() {
            let code = u16::try_from(exc.clone()).unwrap();
            let exc2 = SCloudException::try_from(code).unwrap();
            assert_eq!(exc2, exc, "round-trip cassé pour {exc:?}");
        }
    }
}
