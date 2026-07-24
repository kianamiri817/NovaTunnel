interface StatusDisplayProps {
  connected: boolean;
  provider: string;
}

function StatusDisplay({ connected, provider }: StatusDisplayProps) {
  return (
    <div className="status-display">
      <div className={`status-indicator ${connected ? "connected" : "disconnected"}`}>
        <div className="status-dot" />
        <span className="status-text">
          {connected ? "Connected" : "Disconnected"}
        </span>
      </div>
      {connected && (
        <div className="provider-badge">
          <span className="provider-name">{provider}</span>
        </div>
      )}
    </div>
  );
}

export default StatusDisplay;
