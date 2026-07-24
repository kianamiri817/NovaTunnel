use crate::error::Result;

pub struct FirewallManager {
    kill_switch_enabled: bool,
    rules_applied: bool,
}

impl FirewallManager {
    pub fn new(kill_switch_enabled: bool) -> Self {
        Self {
            kill_switch_enabled,
            rules_applied: false,
        }
    }

    pub async fn apply_rules(&mut self, _tunnel_interface: &str) -> Result<()> {
        if self.kill_switch_enabled {
            tracing::info!("Applying kill switch firewall rules");
            // Platform-specific firewall rules would go here
            // Block all traffic except tunnel interface
            self.rules_applied = true;
        }
        Ok(())
    }

    pub async fn remove_rules(&mut self) -> Result<()> {
        if self.kill_switch_enabled && self.rules_applied {
            tracing::info!("Removing kill switch firewall rules");
            // Platform-specific firewall rule cleanup would go here
            self.rules_applied = false;
        }
        Ok(())
    }

    pub fn is_kill_switch_enabled(&self) -> bool {
        self.kill_switch_enabled
    }

    pub fn set_kill_switch(&mut self, enabled: bool) {
        self.kill_switch_enabled = enabled;
    }
}
