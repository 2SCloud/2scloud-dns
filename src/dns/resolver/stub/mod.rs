use crate::config::Config;
use crate::dns::packet::DNSPacket;
use crate::dns::packet::answer::AnswerSection;
use crate::dns::packet::question::QuestionSection;
use crate::dns::resolver::{check_additional_diff, check_answer_diff, check_authority_diff};
use crate::exceptions::SCloudException;
use std::path::Path;

#[derive(Debug, PartialEq)]
pub struct StubResolver {
    pub(crate) server: std::net::SocketAddr,
    pub(crate) timeout: std::time::Duration,
    pub(crate) retries: u8,
}

impl StubResolver {
    pub fn new(server: std::net::SocketAddr) -> Self {
        let config = Config::from_file(Path::new("./config/config.json")).unwrap();
        Self {
            server,
            timeout: std::time::Duration::from_secs(config.server.graceful_shutdown_timeout_secs),
            retries: 3,
        }
    }

    pub fn resolve(&self, questions: Vec<QuestionSection>) -> Result<DNSPacket, SCloudException> {
        let packet = DNSPacket::new_query(&questions);
        let request_id = packet.header.id;

        let socket = std::net::UdpSocket::bind("0.0.0.0:0")
            .map_err(|_| SCloudException::SCLOUD_STUB_RESOLVER_FAILED_TO_CREATE_SOCKET)?;
        socket
            .set_read_timeout(Some(self.timeout))
            .map_err(|_| SCloudException::SCLOUD_STUB_RESOLVER_FAILED_TO_READ_SOCKET_TIMEOUT)?;

        let bytes = packet.to_bytes()?;
        socket
            .send_to(&bytes, self.server)
            .map_err(|_| SCloudException::SCLOUD_STUB_RESOLVER_FAILED_TO_SEND_TO_SOCKET)?;

        let mut buf = [0u8; 512];

        let mut last_err = None;
        for attempt in 1..=self.retries {
            println!("[STUB_RESOLVER] Attempt {}/{}", attempt, self.retries);
            match socket.recv_from(&mut buf) {
                Ok((size, _)) => {
                    let response = DNSPacket::from_bytes(&buf[..size])?;

                    if response.header.id != request_id {
                        return Err(SCloudException::SCLOUD_STUB_RESOLVER_INVALID_DNS_ID)?;
                    }

                    if !response.header.qr {
                        return Err(SCloudException::SCLOUD_STUB_RESOLVER_INVALID_DNS_RESPONSE)?;
                    }

                    let res_an = check_answer_diff(&questions, &*response.answers);
                    let res_auth = check_authority_diff(&questions, &*response.authorities);
                    let res_add = check_additional_diff(&questions, &*response.additionals);

                    if res_an.is_ok() && res_auth.is_ok() && res_add.is_ok() {
                        return Ok(response);
                    } else {
                        continue;
                    }
                }
                Err(e) => {
                    println!("[STUB_RESOLVER] recv_from error: {:?}", e);
                    if e.kind() == std::io::ErrorKind::WouldBlock
                        || e.kind() == std::io::ErrorKind::TimedOut
                    {
                        last_err = Some(e);
                        continue;
                    } else {
                        return Err(
                            SCloudException::SCLOUD_STUB_RESOLVER_FAILED_TO_RECV_FROM_SOCKET,
                        );
                    }
                }
            }
        }
        Err(SCloudException::SCLOUD_STUB_RESOLVER_FAILED_TO_RECV_FROM_SOCKET)
    }
}
