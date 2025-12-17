use crate::dns::zones::Zone;
use crate::exceptions::SCloudException;
use std::fs::File;
use std::io;
use std::io::BufRead;

pub(crate) fn zone_parser(qname: &str) -> Result<Zone, SCloudException> {
    let mut zone: Zone = Zone {
        origin: None,
        name: "".to_string(),
        ttl: 2500,
        soa: None,
        records: Default::default(),
    };

    let filename = format!("zones/{}.zone", qname);
    let file =
        File::open(&filename).map_err(|_| SCloudException::SCLOUD_ZONE_PARSER_FILE_NOT_FOUND)?;

    for line in io::BufReader::new(file).lines() {
        let line =
            line.map_err(|_| SCloudException::SCLOUD_ZONE_PARSER_FAILED_TO_READ_ZONE_FILE)?;
        if line.starts_with("$ORIGIN") {
            zone.origin = Some(line.split_whitespace().nth(1).unwrap().to_string());
        }
        if line.starts_with("$TTL") {
            zone.ttl = line
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<u16>()
                .map_err(|_| SCloudException::SCLOUD_ZONE_PARSER_FAILED_TO_READ_TTL_FIELD)?;
        }
    }

    Ok(zone)
}
