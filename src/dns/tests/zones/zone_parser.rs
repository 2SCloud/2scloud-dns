#[cfg(test)]
mod tests {
    use crate::dns;
    use crate::dns::zones::Zone;
    use crate::dns::zones::zone_parser::zone_parser;

    #[test]
    fn test_zone_parser() {
        let expected_zone: Zone = Zone {
            origin: Some("nihilist.moe.".to_string()),
            name: "".to_string(),
            ttl: 3600,
            soa: None,
            records: Default::default(),
        };

        let result = zone_parser("nihilist.moe");

        //assert_eq!(result.is_ok(), true);
        assert_eq!(expected_zone, result.unwrap());
    }
}
