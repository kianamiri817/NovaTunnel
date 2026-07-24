use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Provider {
    Warp,
    Nova,
    WireGuard,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DnsMode {
    Auto,
    Custom,
    Secure,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub provider: Provider,
    pub proxy_port: u16,
    pub auto_connect: bool,
    pub kill_switch: bool,
    pub dns_protection: bool,
    pub dns_mode: DnsMode,
    pub custom_dns: Option<String>,
    pub log_level: String,
    pub data_dir: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            provider: Provider::Warp,
            proxy_port: 1080,
            auto_connect: false,
            kill_switch: true,
            dns_protection: true,
            dns_mode: DnsMode::Secure,
            custom_dns: None,
            log_level: "info".to_string(),
            data_dir: None,
        }
    }
}

impl Config {
    pub fn load(path: &std::path::Path) -> anyhow::Result<Self> {
        if path.exists() {
            let content = std::fs::read_to_string(path)?;
            Ok(serde_json::from_str(&content)?)
        } else {
            let config = Self::default();
            config.save(path)?;
            Ok(config)
        }
    }

    pub fn save(&self, path: &std::path::Path) -> anyhow::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}
