use crate::error::Result;
use async_trait::async_trait;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportType {
    Tcp,
    Udp,
    Quic,
    WebSocket,
}

#[async_trait]
pub trait Transport: Send + Sync {
    fn transport_type(&self) -> TransportType;
    async fn connect(&mut self, addr: &str) -> Result<()>;
    async fn disconnect(&mut self) -> Result<()>;
    async fn send(&mut self, data: &[u8]) -> Result<()>;
    async fn receive(&mut self) -> Result<Vec<u8>>;
    fn is_connected(&self) -> bool;
}

pub struct TcpTransport {
    stream: Option<tokio::net::TcpStream>,
    connected: bool,
}

impl Default for TcpTransport {
    fn default() -> Self {
        Self::new()
    }
}

impl TcpTransport {
    pub fn new() -> Self {
        Self {
            stream: None,
            connected: false,
        }
    }
}

#[async_trait]
impl Transport for TcpTransport {
    fn transport_type(&self) -> TransportType {
        TransportType::Tcp
    }

    async fn connect(&mut self, addr: &str) -> Result<()> {
        let stream = tokio::net::TcpStream::connect(addr).await?;
        self.stream = Some(stream);
        self.connected = true;
        Ok(())
    }

    async fn disconnect(&mut self) -> Result<()> {
        self.stream = None;
        self.connected = false;
        Ok(())
    }

    async fn send(&mut self, data: &[u8]) -> Result<()> {
        use tokio::io::AsyncWriteExt;
        if let Some(stream) = &mut self.stream {
            stream.write_all(data).await?;
        }
        Ok(())
    }

    async fn receive(&mut self) -> Result<Vec<u8>> {
        use tokio::io::AsyncReadExt;
        if let Some(stream) = &mut self.stream {
            let mut buf = vec![0u8; 65535];
            let n = stream.read(&mut buf).await?;
            buf.truncate(n);
            Ok(buf)
        } else {
            Err(crate::error::Error::NotConnected)
        }
    }

    fn is_connected(&self) -> bool {
        self.connected
    }
}

pub struct UdpTransport {
    socket: Option<tokio::net::UdpSocket>,
    connected: bool,
}

impl Default for UdpTransport {
    fn default() -> Self {
        Self::new()
    }
}

impl UdpTransport {
    pub fn new() -> Self {
        Self {
            socket: None,
            connected: false,
        }
    }
}

#[async_trait]
impl Transport for UdpTransport {
    fn transport_type(&self) -> TransportType {
        TransportType::Udp
    }

    async fn connect(&mut self, addr: &str) -> Result<()> {
        let socket = tokio::net::UdpSocket::bind("0.0.0.0:0").await?;
        socket.connect(addr).await?;
        self.socket = Some(socket);
        self.connected = true;
        Ok(())
    }

    async fn disconnect(&mut self) -> Result<()> {
        self.socket = None;
        self.connected = false;
        Ok(())
    }

    async fn send(&mut self, data: &[u8]) -> Result<()> {
        if let Some(socket) = &self.socket {
            socket.send(data).await?;
        }
        Ok(())
    }

    async fn receive(&mut self) -> Result<Vec<u8>> {
        if let Some(socket) = &self.socket {
            let mut buf = vec![0u8; 65535];
            let n = socket.recv(&mut buf).await?;
            buf.truncate(n);
            Ok(buf)
        } else {
            Err(crate::error::Error::NotConnected)
        }
    }

    fn is_connected(&self) -> bool {
        self.connected
    }
}
