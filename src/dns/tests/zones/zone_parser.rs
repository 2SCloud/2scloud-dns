#[cfg(test)]
mod tests {
    use crate::dns::q_class::DNSClass;
    use crate::dns::q_type::DNSRecordType;
    use crate::dns::zones::zone_parser::zone_parser;

    #[test]
    fn test_zone_parser() {
        let result = zone_parser("nihilist.moe").unwrap();
        let ns1_record = result.records.get("ns1").unwrap();
        let ns2_record = result.records.get("ns2").unwrap();
        let www_records = result.records.get("www").unwrap();
        let api_records = result.records.get("api").unwrap();
        let blog_records = result.records.get("blog").unwrap();
        let shop_records = result.records.get("shop").unwrap();
        let mail_records = result.records.get("mail").unwrap();
        let backupmail_records = result.records.get("backup-mail").unwrap();
        let xmpp_server_records = result.records.get("_xmpp-server._tcp").unwrap();
        let xmpp_client_records = result.records.get("_xmpp-client._tcp").unwrap();
        let xmpp_records = result.records.get("xmpp").unwrap();
        let ptrtest_records = result.records.get("ptrtest").unwrap();
        let sub_records = result.records.get("sub").unwrap();
        let ns_sub_records = result.records.get("ns.sub").unwrap();

        // ns1 RECORD ASSERT
        assert!(ns1_record.iter().any(|r| r.value == "192.168.1.10"
            && r.name == "ns1"
            && r.rtype == DNSRecordType::A
            && r.rclass == DNSClass::IN
        ));
        assert!(ns1_record.iter().any(|r| r.value == "fd00::10"
            && r.name == "ns1"
            && r.rtype == DNSRecordType::AAAA
            && r.rclass == DNSClass::IN
        ));

        // ns2 RECORD ASSERT
        assert!(ns2_record.iter().any(|r| r.value == "192.168.1.11"
            && r.name == "ns2"
            && r.rtype == DNSRecordType::A
            && r.rclass == DNSClass::IN
        ));
        assert!(ns2_record.iter().any(|r| r.value == "fd00::11"
            && r.name == "ns2"
            && r.rtype == DNSRecordType::AAAA
            && r.rclass == DNSClass::IN
        ));

        // www RECORD ASSERT
        assert!(www_records.iter().any(|r| r.value == "192.168.1.21"
            && r.name == "www"
            && r.rtype == DNSRecordType::A
            && r.rclass == DNSClass::IN
        ));
        assert!(www_records.iter().any(|r| r.value == "fd00::21"
            && r.name == "www"
            && r.rtype == DNSRecordType::AAAA
            && r.rclass == DNSClass::IN
        ));

        // api RECORD ASSERT
        assert!(api_records.iter().any(|r| r.value == "192.168.1.22"
            && r.name == "api"
            && r.rtype == DNSRecordType::A
            && r.rclass == DNSClass::IN
            && r.ttl == 600
        ));

        // blog RECORD ASSERT
        assert!(blog_records.iter().any(|r| r.value == "www"
            && r.name == "blog"
            && r.rtype == DNSRecordType::CNAME
            && r.rclass == DNSClass::IN
        ));

        // shop RECORD ASSERT
        assert!(shop_records.iter().any(|r| r.value == "www.nihilist.moe."
            && r.name == "shop"
            && r.rtype == DNSRecordType::CNAME
            && r.rclass == DNSClass::IN
        ));

        // mail RECORD ASSERT
        assert!(mail_records.iter().any(|r| r.value == "192.168.1.30"
            && r.name == "mail"
            && r.rtype == DNSRecordType::A
            && r.rclass == DNSClass::IN
        ));

        // backup-mail RECORD ASSERT
        assert!(backupmail_records.iter().any(|r| r.value == "192.168.1.31"
            && r.name == "backup-mail"
            && r.rtype == DNSRecordType::A
            && r.rclass == DNSClass::IN
        ));

        // xmpp_server RECORD ASSERT
        assert!(xmpp_server_records.iter().any(|r| r.value == "xmpp.nihilist.moe."
            && r.name == "_xmpp-server._tcp"
            && r.rtype == DNSRecordType::SRV
            && r.rclass == DNSClass::IN
            && r.priority == Some(10)
            && r.weight == Some(5)
            && r.port == Some(5269)
        ));

        // xmpp_client RECORD ASSERT
        assert!(xmpp_client_records.iter().any(|r| r.value == "xmpp.nihilist.moe."
            && r.name == "_xmpp-client._tcp"
            && r.rtype == DNSRecordType::SRV
            && r.rclass == DNSClass::IN
            && r.priority == Some(10)
            && r.weight == Some(5)
            && r.port == Some(5222)
        ));

        // xmpp RECORD ASSERT
        assert!(xmpp_records.iter().any(|r| r.value == "192.168.1.40"
            && r.name == "xmpp"
            && r.rtype == DNSRecordType::A
            && r.rclass == DNSClass::IN
        ));

        // ptrtest RECORD ASSERT
        assert!(ptrtest_records.iter().any(|r| r.value == "example.com."
            && r.name == "ptrtest"
            && r.rtype == DNSRecordType::PTR
            && r.rclass == DNSClass::IN
        ));

        // sub RECORD ASSERT
        assert!(sub_records.iter().any(|r| r.value == "ns.sub.nihilist.moe."
            && r.name == "sub"
            && r.rtype == DNSRecordType::NS
            && r.rclass == DNSClass::IN
        ));

        // sub RECORD ASSERT
        assert!(ns_sub_records.iter().any(|r| r.value == "192.168.2.10"
            && r.name == "ns.sub"
            && r.rtype == DNSRecordType::A
            && r.rclass == DNSClass::IN
        ));
    }
}
