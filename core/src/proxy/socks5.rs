use crate::error::Result;
use crate::tunnel::manager::TunnelManager;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

pub struct Socks5Proxy {
    listener: Option<TcpListener>,
    tunnel_manager: Arc<TunnelManager>,
    running: Arc<AtomicBool>,
}

impl Socks5Proxy {
    pub fn new(tunnel_manager: Arc<TunnelManager>) -> Self {
        Self {
            listener: None,
            tunnel_manager,
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    pub async fn start(&mut self, addr: SocketAddr) -> Result<()> {
        let listener = TcpListener::bind(addr).await?;
        self.listener = Some(listener);
        self.running.store(true, Ordering::SeqCst);

        tracing::info!("SOCKS5 proxy listening on {}", addr);

        Ok(())
    }

    pub async fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
        self.listener = None;
        tracing::info!("SOCKS5 proxy stopped");
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    async fn handle_connection(&self, mut stream: tokio::net::TcpStream) -> Result<()> {
        // SOCKS5 greeting
        let mut buf = [0u8; 2];
        stream.read_exact(&mut buf).await?;

        if buf[0] != 0x05 {
            return Err(crate::error::Error::Protocol(
                "Invalid SOCKS version".to_string(),
            ));
        }

        let nmethods = buf[1] as usize;
        let mut methods = vec![0u8; nmethods];
        stream.read_exact(&mut methods).await?;

        // No authentication required
        stream.write_all(&[0x05, 0x00]).await?;

        // Connection request
        let mut header = [0u8; 4];
        stream.read_exact(&mut header).await?;

        if header[0] != 0x05 {
            return Err(crate::error::Error::Protocol(
                "Invalid SOCKS version".to_string(),
            ));
        }

        let cmd = header[1];
        let atyp = header[3];

        let target_addr = match atyp {
            0x01 => {
                // IPv4
                let mut addr = [0u8; 4];
                stream.read_exact(&mut addr).await?;
                let mut port = [0u8; 2];
                stream.read_exact(&mut port).await?;
                let port = u16::from_be_bytes(port);
                format!("{}.{}.{}.{}:{}", addr[0], addr[1], addr[2], addr[3], port)
            }
            0x03 => {
                // Domain
                let mut len = [0u8; 1];
                stream.read_exact(&mut len).await?;
                let mut domain = vec![0u8; len[0] as usize];
                stream.read_exact(&mut domain).await?;
                let mut port = [0u8; 2];
                stream.read_exact(&mut port).await?;
                let port = u16::from_be_bytes(port);
                format!("{}:{}", String::from_utf8_lossy(&domain), port)
            }
            0x04 => {
                // IPv6
                let mut addr = [0u8; 16];
                stream.read_exact(&mut addr).await?;
                let mut port = [0u8; 2];
                stream.read_exact(&mut port).await?;
                let port = u16::from_be_bytes(port);
                format!("[{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}]:{}", 
                    addr[0], addr[1], addr[2], addr[3], addr[4], addr[5], addr[6], addr[7],
                    addr[8], addr[9], addr[10], addr[11], addr[12], addr[13], addr[14], addr[15],
                    port
                )
            }
            _ => {
                return Err(crate::error::Error::Protocol(
                    "Unsupported address type".to_string(),
                ));
            }
        };

        if cmd == 0x01 {
            // CONNECT command
            tracing::debug!("SOCKS5 CONNECT to {}", target_addr);

            // Forward through tunnel
            if let Ok(target_stream) = tokio::net::TcpStream::connect(&target_addr).await {
                // Success response
                stream
                    .write_all(&[0x05, 0x00, 0x00, 0x01, 0, 0, 0, 0, 0, 0])
                    .await?;

                // Bidirectional forwarding
                let (mut client_read, mut client_write) = stream.into_split();
                let (mut target_read, mut target_write) = target_stream.into_split();

                let client_to_target = tokio::spawn(async move {
                    let mut buf = [0u8; 8192];
                    loop {
                        let n = match client_read.read(&mut buf).await {
                            Ok(0) => break,
                            Ok(n) => n,
                            Err(_) => break,
                        };
                        if target_write.write_all(&buf[..n]).await.is_err() {
                            break;
                        }
                    }
                });

                let target_to_client = tokio::spawn(async move {
                    let mut buf = [0u8; 8192];
                    loop {
                        let n = match target_read.read(&mut buf).await {
                            Ok(0) => break,
                            Ok(n) => n,
                            Err(_) => break,
                        };
                        if client_write.write_all(&buf[..n]).await.is_err() {
                            break;
                        }
                    }
                });

                tokio::select! {
                    _ = client_to_target => {},
                    _ = target_to_client => {},
                }
            } else {
                // Connection refused
                stream
                    .write_all(&[0x05, 0x05, 0x00, 0x01, 0, 0, 0, 0, 0, 0])
                    .await?;
            }
        }

        Ok(())
    }
}
