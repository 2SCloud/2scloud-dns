use crate::config::Config;
use crate::dns::packet::question::QuestionSection;
use crate::dns::q_class::DNSClass;
use crate::dns::q_type::DNSRecordType;
use crate::dns::resolver::stub::StubResolver;
use std::path::Path;

mod config;
mod dns;
mod exceptions;
mod utils;

fn main() {
    let config = Config::from_file(Path::new("./config/config.json")).unwrap();
    // let resolver = StubResolver::new(config.try_get_forwarder_addr_by_index(2, 0).unwrap());
    let resolver = StubResolver::new(
        config
            .try_get_forwarder_addr_by_name("sta-internal")
            .unwrap(),
    );
    println!(
        "{} server is running on port {}...",
        config.server.name, config.server.bind_port,
    );

    let q = vec![QuestionSection {
        q_name: "github.com".to_string(),
        q_type: DNSRecordType::A,
        q_class: DNSClass::IN,
    }];

    let res = resolver.resolve(q);
    println!("{:#?}", res);
}
