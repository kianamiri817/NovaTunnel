use super::connector::WarpConnector;
use crate::error::Result;
use crate::tunnel::provider::{ProviderInfo, ProviderStatus, TunnelProvider, TunnelStats};
use async_trait::async_trait;

pub struct WarpManager {
    status: ProviderStatus,
    connector: WarpConnector,
    stats: TunnelStats,
}

impl WarpManager {
    pub fn new() -> Self {
        Self {
            status: ProviderStatus::Disconnected,
            connector: WarpConnector::new(),
            stats: TunnelStats::default(),
        }
    }

    pub async fn detect_warp() -> bool {
        // Check if WARP is installed and available
        #[cfg(target_os = "windows")]
        {
            std::process::Command::new("warp-cli")
                .arg("status")
                .output()
                .map(|output| output.status.success())
                .unwrap_or(false)
        }
        #[cfg(target_os = "linux")]
        {
            std::process::Command::new("warp-cli")
                .arg("status")
                .output()
                .map(|output| output.status.success())
                .unwrap_or(false)
        }
        #[cfg(target_os = "macos")]
        {
            std::process::Command::new("warp-cli")
                .arg("status")
                .output()
                .map(|output| output.status.success())
                .unwrap_or(false)
        }
        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
        {
            false
        }
    }
}

#[async_trait]
impl TunnelProvider for WarpManager {
    fn name(&self) -> &str {
        "warp"
    }

    fn display_name(&self) -> &str {
        "Cloudflare WARP"
    }

    async fn connect(&mut self) -> Result<()> {
        tracing::info!("Connecting to Cloudflare WARP");
        self.status = ProviderStatus::Connecting;

        self.connector.connect().await?;

        self.status = ProviderStatus::Connected;
        self.stats = TunnelStats::default();

        Ok(())
    }

    async fn disconnect(&mut self) -> Result<()> {
        tracing::info!("Disconnecting from Cloudflare WARP");

        self.connector.disconnect().await?;

        self.status = ProviderStatus::Disconnected;

        Ok(())
    }

    async fn health_check(&self) -> Result<bool> {
        if self.status != ProviderStatus::Connected {
            return Ok(false);
        }

        self.connector.health_check().await
    }

    async fn get_info(&self) -> Result<ProviderInfo> {
        Ok(ProviderInfo {
            name: "warp".to_string(),
            display_name: "Cloudflare WARP".to_string(),
            description: "Free secure tunnel via Cloudflare".to_string(),
            exit_ip: self.connector.get_exit_ip().await.ok(),
            exit_country: None,
            latency_ms: self.stats.latency_ms,
        })
    }

    async fn get_stats(&self) -> Result<TunnelStats> {
        Ok(self.stats.clone())
    }

    fn status(&self) -> ProviderStatus {
        self.status
    }
}
