use std::path::Path;
use crate::config::Config;
use crate::dns::packet::question::QuestionSection;
use crate::dns::q_class::DNSClass;
use crate::dns::q_type::DNSRecordType;
use crate::dns::resolver::stub::StubResolver;

mod dns;
mod exceptions;
mod utils;
mod config;

fn main() {
    let config = Config::from_file(Path::new("./config/config.json")).unwrap();
    let resolver = StubResolver::new(config.forwarder.get(1).unwrap().addresses.get(0).unwrap().parse().unwrap());
    println!("{} server is running on port {}...",
             config.server.name,
             config.server.bind_port,
    );

    let q = vec![QuestionSection {
        q_name: "github.com".to_string(),
        q_type: DNSRecordType::A,
        q_class: DNSClass::IN,
    }];

    let res = resolver.resolve(q);
    println!("{:#?}", res);
}
