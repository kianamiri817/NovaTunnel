#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use novatunnel_core::{
    config::{Config, Provider},
    error::Result,
    event::{Event, EventSender},
    tunnel::{TunnelManager, TunnelStats},
};
use std::sync::Arc;
use tauri::State;

struct AppState {
    tunnel_manager: Arc<TunnelManager>,
    config: Arc<parking_lot::RwLock<Config>>,
    event_sender: EventSender,
}

#[tauri::command]
async fn get_status(state: State<'_, AppState>) -> Result<serde_json::Value> {
    let is_connected = state.tunnel_manager.is_connected();
    let provider_name = state.tunnel_manager.get_provider_name();
    let stats = state.tunnel_manager.get_stats().await.unwrap_or_default();

    Ok(serde_json::json!({
        "connected": is_connected,
        "provider": provider_name.unwrap_or_else(|| "None".to_string()),
        "bytes_sent": stats.bytes_sent,
        "bytes_received": stats.bytes_received,
    }))
}

#[tauri::command]
async fn connect(state: State<'_, AppState>) -> Result<serde_json::Value> {
    state.tunnel_manager.connect().await?;
    Ok(serde_json::json!({ "success": true }))
}

#[tauri::command]
async fn disconnect(state: State<'_, AppState>) -> Result<serde_json::Value> {
    state.tunnel_manager.disconnect().await?;
    Ok(serde_json::json!({ "success": true }))
}

#[tauri::command]
async fn get_config(state: State<'_, AppState>) -> Result<Config> {
    Ok(state.config.read().clone())
}

#[tauri::command]
async fn update_config(state: State<'_, AppState>, config: Config) -> Result<serde_json::Value> {
    *state.config.write() = config;
    Ok(serde_json::json!({ "success": true }))
}

#[tauri::command]
async fn get_stats(state: State<'_, AppState>) -> Result<TunnelStats> {
    state.tunnel_manager.get_stats().await
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let config = Config::load(std::path::Path::new("config.json")).unwrap_or_default();

    let event_sender = EventSender::new(100);
    let tunnel_manager = Arc::new(TunnelManager::new(config.clone(), event_sender.clone()));

    let app_state = AppState {
        tunnel_manager,
        config: Arc::new(parking_lot::RwLock::new(config)),
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
