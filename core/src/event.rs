use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Event {
    Connected { provider: String, exit_ip: String },
    Disconnected { reason: String },
    Connecting { provider: String },
    Error { message: String },
    StatsUpdated { bytes_sent: u64, bytes_received: u64 },
    ProviderChanged { provider: String },
}

#[derive(Clone)]
pub struct EventSender {
    sender: broadcast::Sender<Event>,
}

impl EventSender {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self { sender }
    }

    pub fn send(&self, event: Event) {
        let _ = self.sender.send(event);
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.sender.subscribe()
    }
}
