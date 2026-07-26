#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use novatunnel_core::{config::Provider, config::Config, event::EventSender, providers::warp::WarpManager, tunnel::TunnelManager};
use std::sync::Arc;
use tauri::State;

struct AppState {
    tunnel_manager: Arc<TunnelManager>,
    config: Arc<tokio::sync::RwLock<Config>>,
    config_path: std::path::PathBuf,
    #[allow(dead_code)]
    event_sender: EventSender,
}

fn init_provider(manager: &TunnelManager, config: &Config) {
    let runtime = tokio::runtime::Runtime::new().expect("failed to create tokio runtime");
    match config.provider {
        Provider::Warp => {
            runtime.block_on(async {
                manager.set_provider(Box::new(WarpManager::new())).await;
            });
        }
        Provider::Nova => {
            tracing::warn!("Nova provider is not yet fully implemented");
        }
        Provider::WireGuard => {
            tracing::warn!("WireGuard provider is not yet fully implemented");
        }
    }
}

#[tauri::command]
async fn get_status(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    let is_connected = state.tunnel_manager.is_connected().await;
    let provider_name = state.tunnel_manager.get_provider_name().await;
    let stats = state.tunnel_manager.get_stats().await.unwrap_or_default();

    Ok(serde_json::json!({
        "connected": is_connected,
        "provider": provider_name.unwrap_or_else(|| "None".to_string()),
        "bytes_sent": stats.bytes_sent,
        "bytes_received": stats.bytes_received,
    }))
}

#[tauri::command]
async fn connect(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    state
        .tunnel_manager
        .connect()
        .await
        .map_err(|e| e.to_string())?;
    Ok(serde_json::json!({ "success": true }))
}

#[tauri::command]
async fn disconnect(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    state
        .tunnel_manager
        .disconnect()
        .await
        .map_err(|e| e.to_string())?;
    Ok(serde_json::json!({ "success": true }))
}

#[tauri::command]
async fn get_config(state: State<'_, AppState>) -> Result<Config, String> {
    Ok(state.config.read().await.clone())
}

#[tauri::command]
async fn update_config(
    state: State<'_, AppState>,
    config: Config,
) -> Result<serde_json::Value, String> {
    config.save(&state.config_path).map_err(|e| e.to_string())?;
    *state.config.write().await = config;
    Ok(serde_json::json!({ "success": true }))
}

#[tauri::command]
async fn get_stats(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    let stats = state.tunnel_manager.get_stats().await.unwrap_or_default();
    Ok(serde_json::json!({
        "bytes_sent": stats.bytes_sent,
        "bytes_received": stats.bytes_received,
        "latency_ms": stats.latency_ms,
    }))
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let config_path = std::path::Path::new("config.json").to_path_buf();
    let config = Config::load(&config_path).unwrap_or_default();

    let event_sender = EventSender::new(100);
    let tunnel_manager = Arc::new(TunnelManager::new(config.clone(), event_sender.clone()));

    init_provider(&tunnel_manager, &config);

    let app_state = AppState {
        tunnel_manager,
        config: Arc::new(tokio::sync::RwLock::new(config)),
        config_path,
        event_sender,
    };

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            get_status,
            connect,
            disconnect,
            get_config,
            update_config,
            get_stats,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}