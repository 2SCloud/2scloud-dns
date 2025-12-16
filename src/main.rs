use crate::dns::packet::DNSPacket;
use crate::dns::packet::question::QuestionSection;
use crate::dns::records::{DNSClass, DNSRecordType};
use crate::dns::resolver::stub::resolver::StubResolver;
use std::net::AddrParseError;

mod dns;
mod exceptions;
mod utils;

fn main() {
    let resolver = StubResolver::new("8.8.8.8:53".parse().unwrap());

    let q = vec![QuestionSection {
        q_name: "example.com".to_string(),
        q_type: DNSRecordType::A,
        q_class: DNSClass::IN,
    }];

    let res = resolver.resolve(q);
    println!("{:#?}", res);
}
