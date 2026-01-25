mod config;
mod error;
mod resources;
mod server;
mod tools;

use config::Config;
use error::Result;
use rmcp::{transport::stdio, ServiceExt};
use server::MusicTheoryServer;

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration
    let config = Config::load()?;

    // Initialize logging with twyg from config
    let log_opts = config.logging.to_twyg()?;
    twyg::setup(log_opts)
        .map_err(|e| error::Error::config(format!("Failed to setup logging: {}", e)))?;

    log::info!(
        version = &*config.server.version,
        name = &*config.server.name;
        "Music Theory MCP Server starting"
    );

    // Create and run the MCP server with stdio transport
    log::info!(transport = "stdio"; "Starting MCP server");
    let service = MusicTheoryServer::new(config)
        .serve(stdio())
        .await
        .map_err(|e| {
            error::Error::io(std::io::Error::other(format!(
                "Failed to start server: {:?}",
                e
            )))
        })?;

    // Wait for server to finish
    service
        .waiting()
        .await
        .map_err(|e| error::Error::io(std::io::Error::other(format!("Server error: {:?}", e))))?;

    log::info!("Server stopped");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_loads() {
        let result = Config::load();
        assert!(result.is_ok(), "Failed to load config: {:?}", result.err());
    }
}
