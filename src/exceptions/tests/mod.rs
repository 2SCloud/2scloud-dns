
#[cfg(test)]
mod tests{
    use crate::exceptions::SCloudException;
    use crate::exceptions::SCloudException::SCLOUD_HEADER_DESERIALIZATION_FAILED;

    // EXCEPTIONS
    #[test]
    fn test_exceptions_to_str(){
        let result = SCloudException::to_str(&SCLOUD_HEADER_DESERIALIZATION_FAILED);
        assert_eq!(result, "Buffer length is less than header length.");
    }

}