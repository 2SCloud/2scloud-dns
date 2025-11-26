pub(crate) enum SCloudException {
    // question section
    SCLOUD_QUESTION_IMPOSSIBLE_PARSE_QNAME,
    SCLOUD_QUESTION_DESERIALIZATION_FAILED,
    
    // QTYPE
    SCLOUD_QTYPE_UNKNOWN_TYPE
    
    //QCLASS
}

impl SCloudException {
    fn to_str(&self) -> &'static str {
        match self {
            // question section
            SCloudException::SCLOUD_QUESTION_IMPOSSIBLE_PARSE_QNAME => "Impossible to parse the `q_name`, check if a `q_name is provided.`",
            SCloudException::SCLOUD_QUESTION_DESERIALIZATION_FAILED => "Buffer length is less than question section length.",
        }
    }
}