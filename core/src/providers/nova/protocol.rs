use crate::error::Result;
use bytes::{Buf, BufMut, BytesMut};

pub const NOVA_VERSION: u8 = 1;

#[derive(Debug, Clone)]
pub struct NovaPacket {
    pub version: u8,
    pub session_id: [u8; 32],
    pub timestamp: u64,
    pub flags: u8,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketFlags {
    Handshake = 0x01,
    Data = 0x02,
    KeepAlive = 0x04,
    Disconnect = 0x08,
    Error = 0x10,
}

impl NovaPacket {
    pub fn new(session_id: [u8; 32], flags: u8, payload: Vec<u8>) -> Self {
        Self {
            version: NOVA_VERSION,
            session_id,
            timestamp: chrono::Utc::now().timestamp() as u64,
            flags,
            payload,
        }
    }

    pub fn handshake(session_id: [u8; 32], payload: Vec<u8>) -> Self {
        Self::new(session_id, PacketFlags::Handshake as u8, payload)
    }

    pub fn data(session_id: [u8; 32], payload: Vec<u8>) -> Self {
        Self::new(session_id, PacketFlags::Data as u8, payload)
    }

    pub fn keep_alive(session_id: [u8; 32]) -> Self {
        Self::new(session_id, PacketFlags::KeepAlive as u8, Vec::new())
    }

    pub fn disconnect(session_id: [u8; 32]) -> Self {
        Self::new(session_id, PacketFlags::Disconnect as u8, Vec::new())
    }

    pub fn encode(&self) -> BytesMut {
        let mut buf = BytesMut::with_capacity(45 + self.payload.len());

        buf.put_u8(self.version);
        buf.put_slice(&self.session_id);
        buf.put_u64(self.timestamp);
        buf.put_u8(self.flags);
        buf.put_u16(self.payload.len() as u16);
        buf.put_slice(&self.payload);

        buf
    }

    pub fn decode(data: &[u8]) -> Result<Self> {
        if data.len() < 45 {
            return Err(crate::error::Error::Protocol(
                "Packet too short".to_string(),
            ));
        }

        let mut cursor = data;

        let version = cursor.get_u8();
        if version != NOVA_VERSION {
            return Err(crate::error::Error::Protocol(format!(
                "Unsupported version: {}",
                version
            )));
        }

        let mut session_id = [0u8; 32];
        cursor.copy_to_slice(&mut session_id);

        let timestamp = cursor.get_u64();
        let flags = cursor.get_u8();
        let payload_len = cursor.get_u16() as usize;

        if cursor.remaining() < payload_len {
            return Err(crate::error::Error::Protocol(
                "Incomplete payload".to_string(),
            ));
        }

        let mut payload = vec![0u8; payload_len];
        cursor.copy_to_slice(&mut payload);

        Ok(Self {
            version,
            session_id,
            timestamp,
            flags,
            payload,
        })
    }

    pub fn is_handshake(&self) -> bool {
        self.flags & PacketFlags::Handshake as u8 != 0
    }

    pub fn is_data(&self) -> bool {
        self.flags & PacketFlags::Data as u8 != 0
    }

    pub fn is_keep_alive(&self) -> bool {
        self.flags & PacketFlags::KeepAlive as u8 != 0
    }

    pub fn is_disconnect(&self) -> bool {
        self.flags & PacketFlags::Disconnect as u8 != 0
    }
}

pub struct NovaProtocol {
    session_id: [u8; 32],
}

impl Default for NovaProtocol {
    fn default() -> Self {
        Self::new()
    }
}

impl NovaProtocol {
    pub fn new() -> Self {
        let mut session_id = [0u8; 32];
        use rand::Rng;
        rand::thread_rng().fill(&mut session_id);
        Self { session_id }
    }

    pub fn session_id(&self) -> &[u8; 32] {
        &self.session_id
    }

    pub fn create_handshake_packet(&self, payload: Vec<u8>) -> NovaPacket {
        NovaPacket::handshake(self.session_id, payload)
    }

    pub fn create_data_packet(&self, payload: Vec<u8>) -> NovaPacket {
        NovaPacket::data(self.session_id, payload)
    }

    pub fn create_keep_alive(&self) -> NovaPacket {
        NovaPacket::keep_alive(self.session_id)
    }

    pub fn create_disconnect(&self) -> NovaPacket {
        NovaPacket::disconnect(self.session_id)
    }
}
