use crate::error::Result;

pub struct CryptoManager {
    session_key: Option<[u8; 32]>,
}

impl CryptoManager {
    pub fn new() -> Self {
        Self { session_key: None }
    }

    pub fn set_session_key(&mut self, key: [u8; 32]) {
        self.session_key = Some(key);
    }

    pub fn get_session_key(&self) -> Result<[u8; 32]> {
        self.session_key
            .ok_or(crate::error::Error::Crypto("No session key".to_string()))
    }

    pub fn clear_session_key(&mut self) {
        self.session_key = None;
    }
}
