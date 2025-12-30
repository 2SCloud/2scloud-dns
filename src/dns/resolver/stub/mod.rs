use crate::config::Config;
use crate::dns::packet::DNSPacket;
use crate::dns::packet::question::QuestionSection;
use crate::exceptions::SCloudException;
use std::path::Path;
use crate::dns::packet::answer::AnswerSection;

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

    fn check_answer_diff(questions: &[QuestionSection], answers: &[AnswerSection]) -> Result<bool, SCloudException> {
        let mut found = false;
        for question in questions {

            for answer in answers {
                if answer.q_name == question.q_name
                    && answer.r_type == question.q_type
                    && answer.r_class == question.q_class
                {
                    found = true;
                    break;
                }
            }

            if !found {
                return Err(SCloudException::SCLOUD_STUB_RESOLVER_ANSWER_MISMATCH);
            }
        }
        Ok(found)
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

                    Self::check_answer_diff(&questions, &*response.answers);

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
