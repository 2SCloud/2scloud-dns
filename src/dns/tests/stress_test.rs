use std::path::Path;
use std::time::{Instant, Duration};
use crate::config::Config;
use crate::dns::resolver::stub::StubResolver;
use crate::dns::packet::question::QuestionSection;
use crate::dns::q_type::DNSRecordType;

#[test]
fn stress_test_stub_resolver() {
    let config = Config::from_file(Path::new("./config/config.json")).unwrap();
    let resolver = StubResolver::new(config.try_get_forwarder_addr_by_name("sta-internal").unwrap());

    let domains = vec![
        "github.com",
        "www.google.com",
        "example.com",
        "cloudflare.com",
        "rust-lang.org",
    ];

    let types = vec![
        DNSRecordType::A,
        DNSRecordType::AAAA,
        DNSRecordType::CNAME,
    ];

    let total_queries = 10500;
    let mut success_count = 0;
    let mut failure_count = 0;
    let mut total_duration = Duration::ZERO;
    let overall_start = Instant::now();

    for i in 0..total_queries {
        let domain = domains[i % domains.len()];
        let record_type = types[i % types.len()];

        let question = QuestionSection {
            q_name: domain.to_string(),
            q_type: record_type,
            q_class: crate::dns::q_class::DNSClass::IN,
        };

        let start = Instant::now();
        let result = resolver.resolve(vec![question]);
        let duration = start.elapsed();
        total_duration += duration;

        match result {
            Ok(packet) => {
                success_count += 1;
                println!("[{}] Success {} ({:?})", i, domain, duration);
            }
            Err(e) => {
                failure_count += 1;
                println!("[{}] Failed {}: {:?}", i, domain, e);
            }
        }
    }

    let overall_duration = overall_start.elapsed();

    println!("Stress test finished!");
    println!("Total queries: {}", total_queries);
    println!("Success: {}", success_count);
    println!("Failures: {}", failure_count);
    println!("Average latency: {:?}", total_duration / total_queries as u32);
    println!("Total test duration: {:?}", overall_duration);
}
