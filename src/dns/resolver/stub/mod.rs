use crate::config::Config;
use crate::dns::packet::DNSPacket;
use crate::dns::packet::question::QuestionSection;
use crate::dns::resolver::check_answer_diff;
use crate::exceptions::SCloudException;
use std::path::Path;

/// A simple DNS stub resolver.
///
/// The StubResolver sends DNS queries to an upstream DNS server (typically
/// a recursive resolver) using UDP, waits for a response, validates it,
/// and returns a parsed `DNSPacket`.
///
/// It supports:
/// - configurable timeout
/// - retry logic
/// - basic DNS response validation (ID, QR flag, section consistency)
#[derive(Debug, PartialEq)]
pub struct StubResolver {
    pub(crate) server: std::net::SocketAddr,
    pub(crate) timeout: std::time::Duration,
    pub(crate) retries: u8,
}

impl StubResolver {
    /// Create a new StubResolver targeting the given DNS server.
    ///
    /// Configuration values (timeout, etc.) are loaded from `config/config.json`.
    ///
    /// # Exemple :
    /// ```
    /// use std::net::SocketAddr;
    /// use crate::dns::resolver::StubResolver;
    ///
    /// let server: SocketAddr = "8.8.8.8:53".parse().unwrap();
    /// let resolver = StubResolver::new(server);
    ///
    /// assert_eq!(resolver.server, server);
    /// ```
    pub fn new(server: std::net::SocketAddr) -> Self {
        let config = Config::from_file(Path::new("./config/config.json")).unwrap();
        Self {
            server,
            timeout: std::time::Duration::from_secs(config.server.graceful_shutdown_timeout_secs),
            retries: 3,
        }
    }

    /// Resolve one or more DNS questions using the configured upstream server.
    ///
    /// This function:
    /// - builds a DNS query packet
    /// - sends it over UDP
    /// - waits for a valid DNS response
    /// - retries on timeout
    /// - validates the response ID and sections
    ///
    /// # Exemple :
    /// ```
    /// use std::net::SocketAddr;
    /// use crate::dns::resolver::StubResolver;
    /// use crate::dns::packet::question::QuestionSection;
    /// use crate::dns::q_type::DNSRecordType;
    /// use crate::dns::q_class::DNSClass;
    ///
    /// let resolver = StubResolver::new("8.8.8.8:53".parse::<SocketAddr>().unwrap());
    ///
    /// let questions = vec![QuestionSection {
    ///     q_name: "example.com".to_string(),
    ///     q_type: DNSRecordType::A,
    ///     q_class: DNSClass::IN,
    /// }];
    ///
    /// let response = resolver.resolve(questions).unwrap();
    ///
    /// assert!(response.header.qr); // Must be a response
    /// assert!(!response.answers.is_empty());
    /// ```
    pub fn resolve(&self, questions: Vec<QuestionSection>) -> Result<DNSPacket, SCloudException> {
        let packet = DNSPacket::new_query(&questions.as_slice());
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

                    if let Err(e) = check_answer_diff(
                        &questions,
                        &*response.answers,
                        &*response.authorities,
                        &*response.additionals,
                    ) {
                        return Err(e);
                    }

                    return Ok(response);
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
