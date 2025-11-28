#[derive(Debug)]
pub enum SCloudException {
    // HEADER SECTION
    SCLOUD_HEADER_DESERIALIZATION_FAILED,

    // QUESTION SECTION
    SCLOUD_QUESTION_IMPOSSIBLE_PARSE_QNAME,
    SCLOUD_QUESTION_DESERIALIZATION_FAILED,

    // ANSWER SECTION
    SCLOUD_ANSWER_DESERIALIZATION_FAILED,

    // AUTHORITY SECTION
    SCLOUD_AUTHORITY_DESERIALIZATION_FAILED,

    // ADDITIONAL SECTION
    SCLOUD_ADDITIONAL_DESERIALIZATION_FAILED,

    // QTYPE
    SCLOUD_QTYPE_UNKNOWN_TYPE, //QCLASS
}

impl SCloudException {
    fn to_str(&self) -> &'static str {
        match self {
            //HEADER SECTION
            SCloudException::SCLOUD_HEADER_DESERIALIZATION_FAILED => {
                "Buffer length is less than header length"
            }

            // QUESTION SECTION
            SCloudException::SCLOUD_QUESTION_IMPOSSIBLE_PARSE_QNAME => {
                "Impossible to parse the `q_name`, check if a `q_name is provided.`"
            }
            SCloudException::SCLOUD_QUESTION_DESERIALIZATION_FAILED => {
                "Buffer length is less than question section length."
            }

            // ANSWER SECTION
            SCloudException::SCLOUD_ANSWER_DESERIALIZATION_FAILED => {
                "Buffer length is less than answer section length."
            }

            // AUTHORITY SECTION
            SCloudException::SCLOUD_AUTHORITY_DESERIALIZATION_FAILED => {
                "Buffer length is less than authority section length."
            }

            // ADDITIONAL SECTION
            SCloudException::SCLOUD_ADDITIONAL_DESERIALIZATION_FAILED => {
                "Buffer length is less than additional section length."
            }

            // QTYPE
            SCloudException::SCLOUD_QTYPE_UNKNOWN_TYPE => "Unknown q_type",
        }
    }
}
