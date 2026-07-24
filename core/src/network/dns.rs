use crate::config::DnsMode;
use crate::error::Result;

pub struct DnsManager {
    mode: DnsMode,
    custom_dns: Option<String>,
    protected: bool,
}

impl DnsManager {
    pub fn new(mode: DnsMode, custom_dns: Option<String>, protected: bool) -> Self {
        Self {
            mode,
            custom_dns,
            protected,
        }
    }

    pub fn get_dns_servers(&self) -> Vec<String> {
        match self.mode {
            DnsMode::Auto => vec![
                "1.1.1.1".to_string(),
                "1.0.0.1".to_string(),
            ],
            DnsMode::Secure => vec![
                "1.1.1.1".to_string(),
                "1.0.0.1".to_string(),
            ],
            DnsMode::Custom => {
                if let Some(dns) = &self.custom_dns {
                    vec![dns.clone()]
                } else {
                    vec![
                        "1.1.1.1".to_string(),
                        "1.0.0.1".to_string(),
                    ]
                }
            }
        }
    }

    pub async fn configure_system_dns(&self) -> Result<()> {
        if self.protected {
            tracing::info!("Configuring DNS leak protection");
            // Platform-specific DNS configuration would go here
        }
        Ok(())
    }

    pub async fn restore_system_dns(&self) -> Result<()> {
        if self.protected {
            tracing::info!("Restoring original DNS settings");
            // Platform-specific DNS restore would go here
        }
        Ok(())
    }

    pub fn is_protected(&self) -> bool {
        self.protected
    }
}
