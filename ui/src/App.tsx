import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import ConnectionButton from "./components/ConnectionButton";
import StatusDisplay from "./components/StatusDisplay";
import StatsPanel from "./components/StatsPanel";
import SettingsPanel from "./components/SettingsPanel";

interface TunnelStatus {
  connected: boolean;
  provider: string;
  bytes_sent: number;
  bytes_received: number;
}

interface TunnelStats {
  bytes_sent: number;
  bytes_received: number;
  connection_time: number | null;
  latency_ms: number | null;
}

function App() {
  const [status, setStatus] = useState<TunnelStatus>({
    connected: false,
    provider: "None",
    bytes_sent: 0,
    bytes_received: 0,
  });
  const [stats, setStats] = useState<TunnelStats>({
    bytes_sent: 0,
    bytes_received: 0,
    connection_time: null,
    latency_ms: null,
  });
  const [loading, setLoading] = useState(false);
  const [showSettings, setShowSettings] = useState(false);

  const fetchStatus = async () => {
    try {
      const result = await invoke<TunnelStatus>("get_status");
      setStatus(result);
    } catch (error) {
      console.error("Failed to fetch status:", error);
    }
  };

  const fetchStats = async () => {
    try {
      const result = await invoke<TunnelStats>("get_stats");
      setStats(result);
    } catch (error) {
      console.error("Failed to fetch stats:", error);
    }
  };

  useEffect(() => {
    fetchStatus();
    const interval = setInterval(fetchStatus, 1000);
    return () => clearInterval(interval);
  }, []);

  useEffect(() => {
    if (status.connected) {
      const interval = setInterval(fetchStats, 1000);
      return () => clearInterval(interval);
    }
  }, [status.connected]);

  const handleConnect = async () => {
    setLoading(true);
    try {
      if (status.connected) {
        await invoke("disconnect");
      } else {
        await invoke("connect");
      }
      await fetchStatus();
    } catch (error) {
      console.error("Connection error:", error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="app">
      <div className="header">
        <h1 className="logo">NOVA TUNNEL</h1>
      </div>

      <div className="main-content">
        <StatusDisplay connected={status.connected} provider={status.provider} />
        
        <ConnectionButton
          connected={status.connected}
          loading={loading}
          onClick={handleConnect}
        />

        {status.connected && (
          <StatsPanel stats={stats} />
        )}

        <div className="info-panel">
          <div className="info-row">
            <span className="info-label">Mode</span>
            <span className="info-value">{status.provider}</span>
          </div>
          <div className="info-row">
            <span className="info-label">Proxy</span>
            <span className="info-value">127.0.0.1:1080</span>
          </div>
          <div className="info-row">
            <span className="info-label">Exit</span>
            <span className="info-value">
              {status.connected ? "Cloudflare" : "Direct"}
            </span>
          </div>
        </div>
      </div>

      <div className="footer">
        <button
          className="settings-button"
          onClick={() => setShowSettings(!showSettings)}
        >
          Settings
        </button>
      </div>

      {showSettings && (
        <SettingsPanel onClose={() => setShowSettings(false)} />
      )}
    </div>
  );
}

export default App;
