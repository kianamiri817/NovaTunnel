import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";

interface Config {
  provider: string;
  proxy_port: number;
  auto_connect: boolean;
  kill_switch: boolean;
  dns_protection: boolean;
  dns_mode: string;
  custom_dns: string | null;
  log_level: string;
}

interface SettingsPanelProps {
  onClose: () => void;
}

function SettingsPanel({ onClose }: SettingsPanelProps) {
  const [config, setConfig] = useState<Config>({
    provider: "warp",
    proxy_port: 1080,
    auto_connect: false,
    kill_switch: true,
    dns_protection: true,
    dns_mode: "secure",
    custom_dns: null,
    log_level: "info",
  });
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadConfig();
  }, []);

  const loadConfig = async () => {
    try {
      const result = await invoke<Config>("get_config");
      setConfig(result);
    } catch (error) {
      console.error("Failed to load config:", error);
    } finally {
      setLoading(false);
    }
  };

  const saveConfig = async () => {
    try {
      await invoke("update_config", { config });
      onClose();
    } catch (error) {
      console.error("Failed to save config:", error);
    }
  };

  if (loading) {
    return (
      <div className="settings-overlay">
        <div className="settings-panel">
          <div className="loading">Loading...</div>
        </div>
      </div>
    );
  }

  return (
    <div className="settings-overlay" onClick={onClose}>
      <div className="settings-panel" onClick={(e) => e.stopPropagation()}>
        <div className="settings-header">
          <h2>Settings</h2>
          <button className="close-button" onClick={onClose}>
            ×
          </button>
        </div>

        <div className="settings-content">
          <div className="setting-group">
            <label className="setting-label">Tunnel Mode</label>
            <div className="radio-group">
              <label className="radio-option">
                <input
                  type="radio"
                  name="provider"
                  value="warp"
                  checked={config.provider === "warp"}
                  onChange={(e) => setConfig({ ...config, provider: e.target.value })}
                />
                <span className="radio-label">WARP</span>
                <span className="radio-desc">Free Cloudflare tunnel</span>
              </label>
              <label className="radio-option">
                <input
                  type="radio"
                  name="provider"
                  value="nova"
                  checked={config.provider === "nova"}
                  onChange={(e) => setConfig({ ...config, provider: e.target.value })}
                />
                <span className="radio-label">Nova Protocol</span>
                <span className="radio-desc">Custom secure protocol</span>
              </label>
              <label className="radio-option">
                <input
                  type="radio"
                  name="provider"
                  value="wireguard"
                  checked={config.provider === "wireguard"}
                  onChange={(e) => setConfig({ ...config, provider: e.target.value })}
                />
                <span className="radio-label">WireGuard</span>
                <span className="radio-desc">Fast VPN protocol</span>
              </label>
            </div>
          </div>

          <div className="setting-group">
            <label className="setting-label">Security</label>
            <div className="toggle-group">
              <label className="toggle-option">
                <span className="toggle-label">Kill Switch</span>
                <input
                  type="checkbox"
                  checked={config.kill_switch}
                  onChange={(e) => setConfig({ ...config, kill_switch: e.target.checked })}
                  className="toggle-input"
                />
                <span className="toggle-slider" />
              </label>
              <label className="toggle-option">
                <span className="toggle-label">DNS Protection</span>
                <input
                  type="checkbox"
                  checked={config.dns_protection}
                  onChange={(e) => setConfig({ ...config, dns_protection: e.target.checked })}
                  className="toggle-input"
                />
                <span className="toggle-slider" />
              </label>
              <label className="toggle-option">
                <span className="toggle-label">Auto Connect</span>
                <input
                  type="checkbox"
                  checked={config.auto_connect}
                  onChange={(e) => setConfig({ ...config, auto_connect: e.target.checked })}
                  className="toggle-input"
                />
                <span className="toggle-slider" />
              </label>
            </div>
          </div>

          <div className="setting-group">
            <label className="setting-label">Proxy Port</label>
            <input
              type="number"
              value={config.proxy_port}
              onChange={(e) => setConfig({ ...config, proxy_port: parseInt(e.target.value) || 1080 })}
              className="number-input"
              min="1"
              max="65535"
            />
          </div>
        </div>

        <div className="settings-footer">
          <button className="cancel-button" onClick={onClose}>
            Cancel
          </button>
          <button className="save-button" onClick={saveConfig}>
            Save
          </button>
        </div>
      </div>
    </div>
  );
}

export default SettingsPanel;
