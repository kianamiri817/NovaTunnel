use super::provider::{ProviderStatus, TunnelProvider, TunnelStats};
use super::session::Session;
use crate::config::Config;
use crate::error::{Error, Result};
use crate::event::{Event, EventSender};
use std::sync::Arc;
use tokio::sync::RwLock;

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

    pub async fn set_provider(&self, provider: Box<dyn TunnelProvider>) {
        *self.provider.write().await = Some(provider);
    }

    pub async fn connect(&self) -> Result<()> {
        {
            let mut guard = self.provider.write().await;
            let provider = guard.as_mut().ok_or(Error::ProviderNotAvailable(
                "No provider configured".to_string(),
            ))?;

            if provider.status() == ProviderStatus::Connected {
                return Err(Error::AlreadyConnected);
            }

            self.event_sender.send(Event::Connecting {
                provider: provider.display_name().to_string(),
            });

            provider.connect().await?;
        }

        let (display_name, exit_ip) = {
            let mut guard = self.provider.write().await;
            let provider = guard.as_mut().ok_or(Error::NotConnected)?;
            let info = provider.get_info().await?;
            let name = provider.display_name().to_string();
            let ip = info.exit_ip.unwrap_or_default();
            (name, ip)
        };

        let mut session = Session::new(display_name.clone());
        session.connected(exit_ip.clone());

        *self.session.write().await = Some(session);

        self.event_sender.send(Event::Connected {
            provider: display_name,
            exit_ip,
        });

        Ok(())
    }

    pub async fn disconnect(&self) -> Result<()> {
        {
            let mut guard = self.provider.write().await;
            let provider = guard.as_mut().ok_or(Error::NotConnected)?;

            if provider.status() != ProviderStatus::Connected {
                return Err(Error::NotConnected);
            }

            provider.disconnect().await?;
        }

        *self.session.write().await = None;

        self.event_sender.send(Event::Disconnected {
            reason: "User requested".to_string(),
        });

        Ok(())
    }

    pub async fn get_stats(&self) -> Result<TunnelStats> {
        let guard = self.provider.read().await;
        let provider = guard.as_ref().ok_or(Error::NotConnected)?;
        provider.get_stats().await
    }

    pub async fn is_connected(&self) -> bool {
        self.session.read().await.is_some()
    }

    pub async fn get_provider_name(&self) -> Option<String> {
        self.provider
            .read()
            .await
            .as_ref()
            .map(|p| p.display_name().to_string())
    }
}
