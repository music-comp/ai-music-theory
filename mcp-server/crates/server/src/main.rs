mod config;
mod error;
mod resources;
mod server;
mod tools;

use config::Config;
use error::Result;
use rmcp::{transport::stdio, ServiceExt};
use server::MusicTheoryServer;
use twyg::{Color, ColorAttribute, Colors, LogLevel, OptsBuilder, Output, TSFormat};

#[tokio::main]
async fn main() -> Result<()> {
    // Configure custom colors for twyg
    let mut colors = Colors::default();
    colors.timestamp = Some(Color::fg(ColorAttribute::HiBlack));

    // Initialize logging with twyg (output to stderr for MCP compatibility)
    let log_opts = OptsBuilder::new()
        .coloured(true)
        .level(LogLevel::Info)
        .output(Output::Stderr)
        .timestamp_format(TSFormat::Simple)
        .colors(colors)
        .build()
        .map_err(|e| error::Error::config(format!("Failed to build log options: {}", e)))?;

    twyg::setup(log_opts)
        .map_err(|e| error::Error::config(format!("Failed to setup logging: {}", e)))?;

    // Load configuration
    let config = Config::load()?;

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
            error::Error::io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to start server: {:?}", e),
            ))
        })?;

    // Wait for server to finish
    service.waiting().await.map_err(|e| {
        error::Error::io(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Server error: {:?}", e),
        ))
    })?;

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
