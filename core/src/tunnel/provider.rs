use async_trait::async_trait;
use crate::error::Result;

#[derive(Debug, Clone)]
pub struct ProviderInfo {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub exit_ip: Option<String>,
    pub exit_country: Option<String>,
    pub latency_ms: Option<u64>,
}

#[derive(Debug, Clone, Default)]
pub struct TunnelStats {
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub connection_time: Option<std::time::Duration>,
    pub latency_ms: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderStatus {
    Disconnected,
    Connecting,
    Connected,
    Error,
}

#[async_trait]
pub trait TunnelProvider: Send + Sync {
    fn name(&self) -> &str;
    fn display_name(&self) -> &str;
    
    async fn connect(&mut self) -> Result<()>;
    async fn disconnect(&mut self) -> Result<()>;
    async fn health_check(&self) -> Result<bool>;
    async fn get_info(&self) -> Result<ProviderInfo>;
    async fn get_stats(&self) -> Result<TunnelStats>;
    fn status(&self) -> ProviderStatus;
}
