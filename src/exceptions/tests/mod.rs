
#[cfg(test)]
mod tests{
    use strum::IntoEnumIterator;
    use crate::exceptions::SCloudException;

    #[test]
    fn test_exceptions_to_str(){
        let mut i: usize = 0;
        let ex_msg_array: [&'static str; 16] = [
            //HEADER SECTION
            "Buffer length is less than header length.",
            "The header is empty.",
            // QUESTION SECTION
            "Buffer length is less than question section length.",
            "`q_name` too long.",
            // ANSWER SECTION
            "Buffer length is less than answer section length.",
            "Label too long for DNS.",
            "Impossible to deserialize, `buf.len()` is lower than `pos+10`.",
            "Impossible to deserialize, `buf.len()` is lower than `pos+rdlength`.",
            // AUTHORITY SECTION
            "Buffer length is less than authority section length.",
            "Impossible to deserialize, `buf.len()` is lower than `pos+10`.",
            // ADDITIONAL SECTION
            "Buffer length is less than additional section length.",
            // QNAME
            "Impossible to parse the `q_name`, check if a `q_name` is provided.",
            "Impossible to parse the `q_name`, pos is greater than buffer length.",
            "Impossible to parse the `q_name`, pos and len are greater than buffer length.",
            "Impossible to parse the `q_name`, compression 0xC0xx failed.",
            // QTYPE
            "Unknown `q_type`."
        ];
        for ex in SCloudException::iter() {
            println!("expected: {:?}\ngot: {}", ex_msg_array[i], SCloudException::to_str(&ex));
            assert_eq!(SCloudException::to_str(&ex), ex_msg_array[i]);
            i+=1;
        }
    }

}