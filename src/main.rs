use crate::dns::packet::question::QuestionSection;
use crate::dns::q_class::DNSClass;
use crate::dns::q_type::DNSRecordType;
use crate::dns::resolver::stub::StubResolver;

mod dns;
mod exceptions;
mod utils;
mod config;

fn main() {
    let resolver = StubResolver::new("192.0.0.245:53".parse().unwrap());

    let q = vec![QuestionSection {
        q_name: "github.com".to_string(),
        q_type: DNSRecordType::A,
        q_class: DNSClass::IN,
    }];

    let res = resolver.resolve(q);
    println!("{:#?}", res);
}
