use crate::dns::packet::DNSPacket;
use crate::dns::packet::question::QuestionSection;
use crate::exceptions::SCloudException;

pub struct StubResolver {
    server: std::net::SocketAddr,
    timeout: std::time::Duration,
    retries: u8,
}

impl StubResolver {
    pub fn new(server: std::net::SocketAddr) -> Self {
        Self {
            server,
            timeout: std::time::Duration::from_secs(2),
            retries: 2,
        }
    }

    pub fn resolve(&self, questions: Vec<QuestionSection>) -> Result<DNSPacket, SCloudException> {
        let packet = DNSPacket::new_query(questions);
        let request_id = packet.header.id;

        let socket = std::net::UdpSocket::bind("0.0.0.0:0")
            .map_err(|_| SCloudException::SCLOUD_STUB_RESOLVER_FAILED_TO_CREATE_SOCKET)?;
        socket
            .set_read_timeout(Some(std::time::Duration::from_secs(2)))
            .map_err(|_| SCloudException::SCLOUD_STUB_RESOLVER_FAILED_TO_READ_SOCKET_TIMEOUT)?;

        let bytes = packet.to_bytes()?;
        socket
            .send_to(&bytes, self.server)
            .map_err(|_| SCloudException::SCLOUD_STUB_RESOLVER_FAILED_TO_SEND_TO_SOCKET)?;

        let mut buf = [0u8; 512];
        let (size, _) = socket
            .recv_from(&mut buf)
            .map_err(|_| SCloudException::SCLOUD_STUB_RESOLVER_FAILED_TO_RECV_FROM_SOCKET)?;

        let response = DNSPacket::from_bytes(&buf[..size])?;

        if response.header.id != request_id {
            return Err(SCloudException::SCLOUD_STUB_RESOLVER_INVALID_DNS_ID);
        }

        if response.header.qr == false {
            return Err(SCloudException::SCLOUD_STUB_RESOLVER_INVALID_DNS_RESPONSE);
        }

        Ok(response)
    }
}
