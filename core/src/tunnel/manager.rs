use super::provider::{ProviderStatus, TunnelProvider, TunnelStats};
use super::session::Session;
use crate::config::Config;
use crate::error::{Error, Result};
use crate::event::{Event, EventSender};
use parking_lot::RwLock;
use std::sync::Arc;

pub struct TunnelManager {
    #[allow(dead_code)]
    config: Arc<RwLock<Config>>,
    provider: Arc<RwLock<Option<Box<dyn TunnelProvider>>>>,
    session: Arc<RwLock<Option<Session>>>,
    event_sender: EventSender,
}

impl TunnelManager {
    pub fn new(config: Config, event_sender: EventSender) -> Self {
        Self {
            config: Arc::new(RwLock::new(config)),
            provider: Arc::new(RwLock::new(None)),
            session: Arc::new(RwLock::new(None)),
            event_sender,
        }
    }

    pub fn set_provider(&self, provider: Box<dyn TunnelProvider>) {
        *self.provider.write() = Some(provider);
    }

    pub async fn connect(&self) -> Result<()> {
        let display_name = {
            let mut provider = self.provider.write();
            let provider = provider.as_mut().ok_or(Error::ProviderNotAvailable(
                "No provider configured".to_string(),
            ))?;

            if provider.status() == ProviderStatus::Connected {
                return Err(Error::AlreadyConnected);
            }

            self.event_sender.send(Event::Connecting {
                provider: provider.display_name().to_string(),
            });

            provider.connect().await?;
            provider.display_name().to_string()
        };

        let (info, provider_display_name) = {
            let mut provider_guard = self.provider.write();
            let provider = provider_guard.as_mut().ok_or(Error::NotConnected)?;
            let info = provider.get_info().await?;
            (info, provider.display_name().to_string())
        };

        let exit_ip = info.exit_ip.clone().unwrap_or_default();
        let mut session = Session::new(display_name);
        session.connected(exit_ip.clone());

        *self.session.write() = Some(session);

        self.event_sender.send(Event::Connected {
            provider: provider_display_name,
            exit_ip,
        });

        Ok(())
    }

    pub async fn disconnect(&self) -> Result<()> {
        {
            let mut provider = self.provider.write();
            let provider = provider.as_mut().ok_or(Error::NotConnected)?;

            if provider.status() != ProviderStatus::Connected {
                return Err(Error::NotConnected);
            }

            provider.disconnect().await?;
        }

        *self.session.write() = None;

        self.event_sender.send(Event::Disconnected {
            reason: "User requested".to_string(),
        });

        Ok(())
    }

    pub async fn get_stats(&self) -> Result<TunnelStats> {
        let stats = {
            let provider = self.provider.read();
            let provider = provider.as_ref().ok_or(Error::NotConnected)?;
            provider.get_stats().await?
        };
        Ok(stats)
    }

    pub fn is_connected(&self) -> bool {
        self.session.read().is_some()
    }

    pub fn get_provider_name(&self) -> Option<String> {
        self.provider
            .read()
            .as_ref()
            .map(|p| p.display_name().to_string())
    }
}
