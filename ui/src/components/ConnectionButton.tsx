interface ConnectionButtonProps {
  connected: boolean;
  loading: boolean;
  onClick: () => void;
}

function ConnectionButton({ connected, loading, onClick }: ConnectionButtonProps) {
  return (
    <button
      className={`connection-button ${connected ? "connected" : "disconnected"} ${loading ? "loading" : ""}`}
      onClick={onClick}
      disabled={loading}
    >
      <div className="button-content">
        {loading ? (
          <div className="spinner" />
        ) : (
          <div className={`power-icon ${connected ? "on" : "off"}`} />
        )}
        <span className="button-text">
          {loading ? "Connecting..." : connected ? "Disconnect" : "Connect"}
        </span>
      </div>
    </button>
  );
}

export default ConnectionButton;
