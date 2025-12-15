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
    SCLOUD_QTYPE_UNKNOWN_TYPE, //QCLASS
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
        }
    }
}
