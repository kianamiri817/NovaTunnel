use crate::error::Result;

pub struct RouteManager {
    original_gateway: Option<String>,
    tunnel_interface: Option<String>,
}

impl RouteManager {
    pub fn new() -> Self {
        Self {
            original_gateway: None,
            tunnel_interface: None,
        }
    }

    pub async fn setup_routes(&mut self, tunnel_interface: &str) -> Result<()> {
        tracing::info!(
            "Setting up routes for tunnel interface: {}",
            tunnel_interface
        );
        self.tunnel_interface = Some(tunnel_interface.to_string());
        // Platform-specific route setup would go here
        Ok(())
    }

    pub async fn cleanup_routes(&mut self) -> Result<()> {
        tracing::info!("Cleaning up routes");
        self.tunnel_interface = None;
        // Platform-specific route cleanup would go here
        Ok(())
    }

    pub fn is_tunneled(&self, dest_ip: &str) -> bool {
        // Check if destination should go through tunnel
        // For now, all traffic goes through tunnel
        let _ = dest_ip;
        true
    }
}
