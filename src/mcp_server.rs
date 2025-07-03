// MCP server core module (multi-transport entrypoint)
use std::sync::Arc;
use crate::config::{load_config, AppConfig};
use crate::logging::init_logging;
use crate::auth_manager::AuthManager;
use crate::google_api_client::GoogleApiClient;
use crate::error::McpError;

pub struct McpServer {
    pub config: Arc<AppConfig>,
    pub auth_manager: Arc<AuthManager>,
    pub api_client: Arc<GoogleApiClient>,
}

impl McpServer {
    pub async fn new() -> Result<Self, McpError> {
        // Initialize logging
        init_logging();
        // Load config
        let config = Arc::new(load_config().map_err(|e| McpError::Other(e.to_string()))?);
        // Initialize AuthManager
        let auth_manager = Arc::new(AuthManager::new(&config).await?);
        // Initialize GoogleApiClient
        let api_client = Arc::new(GoogleApiClient::new(auth_manager.clone()).await?);
        Ok(McpServer { config, auth_manager, api_client })
    }

    // Entrypoint for starting all transports (HTTP, stdio, SEE)
    pub async fn run_all_transports(&self) -> Result<(), McpError> {
        // Start HTTP, stdio, and SEE servers in parallel (tokio tasks)
        // Pass Arc<GoogleApiClient> to each
        // ...implementation to spawn servers...
        Ok(())
    }
}
