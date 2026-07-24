interface TunnelStats {
  bytes_sent: number;
  bytes_received: number;
  connection_time: number | null;
  latency_ms: number | null;
}

interface StatsPanelProps {
  stats: TunnelStats;
}

function formatBytes(bytes: number): string {
  if (bytes === 0) return "0 B";
  const k = 1024;
  const sizes = ["B", "KB", "MB", "GB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
}

function StatsPanel({ stats }: StatsPanelProps) {
  return (
    <div className="stats-panel">
      <div className="stat-item">
        <span className="stat-label">Upload</span>
        <span className="stat-value upload">{formatBytes(stats.bytes_sent)}</span>
      </div>
      <div className="stat-item">
        <span className="stat-label">Download</span>
        <span className="stat-value download">{formatBytes(stats.bytes_received)}</span>
      </div>
      {stats.latency_ms !== null && (
        <div className="stat-item">
          <span className="stat-label">Latency</span>
          <span className="stat-value">{stats.latency_ms}ms</span>
        </div>
      )}
    </div>
  );
}

export default StatsPanel;
