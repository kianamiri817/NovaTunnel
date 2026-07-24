use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Session {
    pub id: Uuid,
    pub provider: String,
    pub connected_at: Option<DateTime<Utc>>,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub exit_ip: Option<String>,
}

impl Session {
    pub fn new(provider: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            provider,
            connected_at: None,
            bytes_sent: 0,
            bytes_received: 0,
            exit_ip: None,
        }
    }

    pub fn connected(&mut self, exit_ip: String) {
        self.connected_at = Some(Utc::now());
        self.exit_ip = Some(exit_ip);
    }

    pub fn update_stats(&mut self, sent: u64, received: u64) {
        self.bytes_sent = sent;
        self.bytes_received = received;
    }
}
