mod config;
mod error;
mod resources;
mod server;
mod tools;

use config::Config;
use error::Result;
use rmcp::ServiceExt;
use server::MusicTheoryServer;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();

    // Load configuration
    let config = Config::load()?;

    log::info!("Music Theory MCP Server v{}", config.server.version);
    log::info!("Server: {}", config.server.name);

    // Create server instance
    let server = MusicTheoryServer::new(config);

    // Set up stdio transport
    let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());

    // Start the MCP server
    log::info!("Starting MCP server with stdio transport...");
    let running_server = server.serve::<_, std::io::Error, _>((stdin, stdout)).await
        .map_err(|e| error::Error::io(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to start server: {:?}", e)
        )))?;

    // Wait for server to finish
    let quit_reason = running_server.waiting().await
        .map_err(|e| error::Error::io(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Server error: {:?}", e)
        )))?;

    log::info!("Server stopped: {:?}", quit_reason);

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
