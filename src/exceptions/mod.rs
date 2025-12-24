use strum_macros::EnumIter;

mod tests;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, EnumIter, Clone)]
pub enum SCloudException {
    // HEADER SECTION
    SCLOUD_HEADER_DESERIALIZATION_FAILED,
    SCLOUD_HEADER_BYTES_EMPTY,

    // QUESTION SECTION
    SCLOUD_QUESTION_DESERIALIZATION_FAILED,
    SCLOUD_QUESTION_SERIALIZATION_FAILED_QNAME_TOO_LONG,

    // ANSWER SECTION
    SCLOUD_ANSWER_DESERIALIZATION_FAILED,
    SCLOUD_ANSWER_DESERIALIZATION_FAILED_LABEL_TOO_LONG,
    SCLOUD_IMPOSSIBLE_PARSE_ANSWER_HEADER_TOO_SHORT,
    SCLOUD_IMPOSSIBLE_PARSE_ANSWER_RDATA_OUT_OF_BOUNDS,

    // AUTHORITY SECTION
    SCLOUD_AUTHORITY_DESERIALIZATION_FAILED_RDATA_OUT_OF_BOUNDS,
    SCLOUD_AUTHORITY_DESERIALIZATION_FAILED_BUF_TOO_SHORT,

    // ADDITIONAL SECTION
    SCLOUD_ADDITIONAL_DESERIALIZATION_FAILED,
    SCLOUD_ADDITIONAL_DESERIALIZATION_FAILED_BUF_TOO_SHORT,
    SCLOUD_ADDITIONAL_DESERIALIZATION_FAILED_QNAME_TOO_LONG,
    SCLOUD_ADDITIONAL_DESERIALIZATION_FAILED_RDATA_OUT_OF_BOUNDS,

    // QNAME
    SCLOUD_IMPOSSIBLE_PARSE_QNAME,
    SCLOUD_IMPOSSIBLE_PARSE_QNAME_POS_GREATER_THAN_BUF,
    SCLOUD_IMPOSSIBLE_PARSE_QNAME_POS_AND_LEN_GREATER_THAN_BUF,
    SCLOUD_IMPOSSIBLE_PARSE_QNAME_COMPRESSION_FAILED,

    // QTYPE
    SCLOUD_QTYPE_UNKNOWN_TYPE,

    //QCLASS
    SCLOUD_QCLASS_U16_FOR_DNSCLASS_UNKNOWN,
    SCLOUD_QCLASS_DNSCLASS_FOR_U16_UNKNOWN,

    // STUB RESOLVER
    SCLOUD_STUB_RESOLVER_INVALID_DNS_ID,
    SCLOUD_STUB_RESOLVER_INVALID_DNS_RESPONSE,
    SCLOUD_STUB_RESOLVER_FAILED_TO_CREATE_SOCKET,
    SCLOUD_STUB_RESOLVER_FAILED_TO_READ_SOCKET_TIMEOUT,
    SCLOUD_STUB_RESOLVER_FAILED_TO_SEND_TO_SOCKET,
    SCLOUD_STUB_RESOLVER_FAILED_TO_RECV_FROM_SOCKET,

    // ZONES
    SCLOUD_ZONE_PARSER_FILE_NOT_FOUND,
    SCLOUD_ZONE_PARSER_FILE_EMPTY,
    SCLOUD_ZONE_PARSER_FAILED_TO_READ_ZONE_FILE,
    SCLOUD_ZONE_PARSER_FAILED_TO_READ_TTL_FIELD,

    // CONFIG
    SCLOUD_CONFIG_FILE_NOT_FOUND,
    SCLOUD_CONFIG_IMPOSSIBLE_TO_PARSE_JSON,
    SCLOUD_CONFIG_MISSING_FORWARDER,
    SCLOUD_CONFIG_MISSING_ADDRESS,
    SCLOUD_CONFIG_IMPOSSIBLE_TO_PARSE_ADDR,
}

impl SCloudException {
    pub(crate) fn to_str(&self) -> &'static str {
        match self {
            //HEADER SECTION
            SCloudException::SCLOUD_HEADER_DESERIALIZATION_FAILED => {
                "Buffer length is less than header length."
            }

            SCloudException::SCLOUD_HEADER_BYTES_EMPTY => "The header is empty.",

            // QUESTION SECTION
            SCloudException::SCLOUD_QUESTION_DESERIALIZATION_FAILED => {
                "Buffer length is less than question section length."
            }
            SCloudException::SCLOUD_QUESTION_SERIALIZATION_FAILED_QNAME_TOO_LONG => {
                "`q_name` too long."
            }

            // ANSWER SECTION
            SCloudException::SCLOUD_ANSWER_DESERIALIZATION_FAILED => {
                "Buffer length is less than answer section length."
            }
            SCloudException::SCLOUD_ANSWER_DESERIALIZATION_FAILED_LABEL_TOO_LONG => {
                "Label too long for DNS."
            }
            SCloudException::SCLOUD_IMPOSSIBLE_PARSE_ANSWER_HEADER_TOO_SHORT => {
                "Failed to parse DNS answer section: not enough bytes for TYPE, CLASS, TTL, and RDLENGTH."
            }
            SCloudException::SCLOUD_IMPOSSIBLE_PARSE_ANSWER_RDATA_OUT_OF_BOUNDS => {
                "Failed to parse DNS answer section: RDLENGTH exceeds remaining buffer size."
            }

            // AUTHORITY SECTION
            SCloudException::SCLOUD_AUTHORITY_DESERIALIZATION_FAILED_RDATA_OUT_OF_BOUNDS => {
                "Buffer length is less than authority section length."
            }
            SCloudException::SCLOUD_AUTHORITY_DESERIALIZATION_FAILED_BUF_TOO_SHORT => {
                "Impossible to deserialize, `buf.len()` is lower than `pos+10`. (buf too short)"
            }

            // ADDITIONAL SECTION
            SCloudException::SCLOUD_ADDITIONAL_DESERIALIZATION_FAILED => {
                "Buffer length is less than additional section length."
            }
            SCloudException::SCLOUD_ADDITIONAL_DESERIALIZATION_FAILED_BUF_TOO_SHORT => {
                "Impossible to deserialize, `buf.len()` is lower than `pos+10`. (buf too short)"
            }
            SCloudException::SCLOUD_ADDITIONAL_DESERIALIZATION_FAILED_QNAME_TOO_LONG => {
                "`q_name` too long."
            }
            SCloudException::SCLOUD_ADDITIONAL_DESERIALIZATION_FAILED_RDATA_OUT_OF_BOUNDS => {
                "Buffer length is less than authority section length."
            }

            // QNAME
            SCloudException::SCLOUD_IMPOSSIBLE_PARSE_QNAME => {
                "Impossible to parse the `q_name`, check if a `q_name` is provided."
            }
            SCloudException::SCLOUD_IMPOSSIBLE_PARSE_QNAME_POS_GREATER_THAN_BUF => {
                "Impossible to parse the `q_name`, pos is greater than buffer length."
            }
            SCloudException::SCLOUD_IMPOSSIBLE_PARSE_QNAME_POS_AND_LEN_GREATER_THAN_BUF => {
                "Impossible to parse the `q_name`, pos and len are greater than buffer length."
            }
            SCloudException::SCLOUD_IMPOSSIBLE_PARSE_QNAME_COMPRESSION_FAILED => {
                "Impossible to parse the `q_name`, compression 0xC0xx failed."
            }

            // QTYPE
            SCloudException::SCLOUD_QTYPE_UNKNOWN_TYPE => "Unknown `q_type`.",

            //QCLASS
            SCloudException::SCLOUD_QCLASS_U16_FOR_DNSCLASS_UNKNOWN => "Unknown `q_class`.",
            SCloudException::SCLOUD_QCLASS_DNSCLASS_FOR_U16_UNKNOWN => "Unknown `q_class`.",

            // STUB RESOLVER
            SCloudException::SCLOUD_STUB_RESOLVER_INVALID_DNS_ID => {
                "[STUB_RESOLVER] Invalid DNS ID (difference between `response.header.id` and `request_id`)."
            }
            SCloudException::SCLOUD_STUB_RESOLVER_INVALID_DNS_RESPONSE => {
                "[STUB_RESOLVER] Invalid DNS response."
            }
            SCloudException::SCLOUD_STUB_RESOLVER_FAILED_TO_CREATE_SOCKET => {
                "[STUB_RESOLVER] Failed to create UDP socket."
            }
            SCloudException::SCLOUD_STUB_RESOLVER_FAILED_TO_READ_SOCKET_TIMEOUT => {
                "[STUB_RESOLVER] Failed to read, socket timeout."
            }
            SCloudException::SCLOUD_STUB_RESOLVER_FAILED_TO_SEND_TO_SOCKET => {
                "[STUB_RESOLVER] Failed to send to socket."
            }
            SCloudException::SCLOUD_STUB_RESOLVER_FAILED_TO_RECV_FROM_SOCKET => {
                "[STUB_RESOLVER] Failed to receive from socket."
            }

            // ZONES
            SCloudException::SCLOUD_ZONE_PARSER_FILE_NOT_FOUND => {
                "[ZONE_PARSER] Zone file not found."
            }
            SCloudException::SCLOUD_ZONE_PARSER_FILE_EMPTY => "[ZONE_PARSER] Zone file is empty.",
            SCloudException::SCLOUD_ZONE_PARSER_FAILED_TO_READ_ZONE_FILE => {
                "[ZONE_PARSER] `zone_parser()` failed to read the zone file."
            }
            SCloudException::SCLOUD_ZONE_PARSER_FAILED_TO_READ_TTL_FIELD => {
                "[ZONE_PARSER] `zone_parser()` detect TTL field but failed to read this field."
            }

            //CONFIG
            SCloudException::SCLOUD_CONFIG_FILE_NOT_FOUND => {
                "[SCLOUD_CONFIG] Configuration file not found."
            }
            SCloudException::SCLOUD_CONFIG_IMPOSSIBLE_TO_PARSE_JSON => {
                "[SCLOUD_CONFIG] Error while parsing the JSON file."
            }
            SCloudException::SCLOUD_CONFIG_MISSING_FORWARDER => {
                "[SCLOUD_CONFIG] Missing forwarder."
            }
            SCloudException::SCLOUD_CONFIG_MISSING_ADDRESS => "[SCLOUD_CONFIG] Missing address.",
            SCloudException::SCLOUD_CONFIG_IMPOSSIBLE_TO_PARSE_ADDR => {
                "[SCLOUD_CONFIG] Error while parsing the IP address."
            }
        }
    }
}
