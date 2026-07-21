#[cfg(test)]
use crate::utils;

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_with_random_id_with_extension() {
        let original = "file.txt";
        let result = utils::uuid::with_random_id(original);

        assert!(result.starts_with("file-"));
        assert!(result.ends_with(".txt"));

        let without_ext = result.strip_suffix(".txt").unwrap();
        let uuid_part = without_ext.strip_prefix("file-").unwrap();

        assert!(Uuid::parse_str(uuid_part).is_ok());
    }

    #[test]
    fn test_with_random_id_without_extension() {
        let original = "file";
        let result = utils::uuid::with_random_id(original);

        assert!(result.starts_with("file-"));

        let uuid_part = result.strip_prefix("file-").unwrap();

        assert!(Uuid::parse_str(uuid_part).is_ok());
    }

    #[test]
    fn test_with_random_id_multiple_dots() {
        let original = "archive.tar.gz";
        let result = utils::uuid::with_random_id(original);

        assert!(result.ends_with(".gz"));
        assert!(result.starts_with("archive.tar-"));

        let without_ext = result.strip_suffix(".gz").unwrap();
        let uuid_part = without_ext.strip_prefix("archive.tar-").unwrap();

        assert!(Uuid::parse_str(uuid_part).is_ok());
    }

    #[test]
    fn test_generate_uuid_validity() {
        let uuid = utils::uuid::generate_uuid();
        let uuid_str = uuid.to_string();

        assert!(Uuid::parse_str(&uuid_str).is_ok());
    }

    #[test]
    fn test_generate_uuid_is_v4() {
        let uuid = utils::uuid::generate_uuid();

        assert_eq!(uuid.get_version_num(), 4);
    }

    #[test]
    fn test_generate_uuid_uniqueness() {
        let uuid1 = utils::uuid::generate_uuid();
        let uuid2 = utils::uuid::generate_uuid();

        assert_ne!(uuid1, uuid2);
    }

    #[test]
    fn test_uuid_as_static_str_content() {
        let uuid = Uuid::new_v4();
        let expected = uuid.to_string();

        let static_str = utils::uuid::uuid_as_static_str(uuid);

        assert_eq!(static_str, expected);
    }

    #[test]
    fn test_uuid_as_static_str_is_valid_uuid() {
        let uuid = Uuid::new_v4();
        let static_str = utils::uuid::uuid_as_static_str(uuid);

        assert!(Uuid::parse_str(static_str).is_ok());
    }
}
