mod config;
mod error;
mod markdown;
mod metadata;
mod resources;
mod search;
mod server;
mod tools;
mod util;

use config::Config;
use error::Result;
use rmcp::{transport::stdio, ServiceExt};
use server::MusicTheoryServer;

/// Add context to a configuration error.
fn config_context(context: &str, error: impl std::fmt::Display) -> error::Error {
    error::Error::config(format!("{}: {}", context, error))
}

/// Add context to an IO/server error.
fn io_context(context: &str, error: impl std::fmt::Debug) -> error::Error {
    error::Error::io(std::io::Error::other(format!("{}: {:?}", context, error)))
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration
    let config = Config::load()?;

    // Initialize logging with twyg from config
    let log_opts = config.logging.to_twyg()?;
    twyg::setup(log_opts).map_err(|e| config_context("Failed to setup logging", e))?;

    log::debug!(
        "Path resolution: binary={:?}, server_root={:?}, config_dir={:?}, skill_root={:?}",
        util::paths::binary_path(),
        util::paths::server_root(),
        util::paths::config_dir(),
        util::paths::skill_root()
    );

    log::info!(
        version = &*config.server.version,
        name = &*config.server.name;
        "Music Theory MCP Server starting"
    );

    // Build Tantivy index if configured (only when fts feature enabled)
    #[cfg(feature = "fts")]
    if config.search.backend == "tantivy" && config.search.rebuild_on_startup {
        log::info!("Rebuilding Tantivy index on startup");
        match search::build_index(&config).await {
            Ok(stats) => {
                log::info!(
                    files = stats.files_found,
                    indexed = stats.indexed,
                    errors = stats.errors;
                    "Index build complete"
                );
            }
            Err(e) => {
                log::error!("Failed to build index: {}", e);
                return Err(e);
            }
        }
    } else if config.search.backend == "tantivy" {
        log::info!("Using existing Tantivy index (rebuild_on_startup = false)");
    }

    // Create and run the MCP server with stdio transport
    log::info!(transport = "stdio"; "Starting MCP server");
    let service = MusicTheoryServer::new(config)
        .serve(stdio())
        .await
        .map_err(|e| io_context("Failed to start server", e))?;

    // Set up graceful shutdown handler for Ctrl+C
    let cancel_token = service.cancellation_token();
    tokio::spawn(async move {
        match tokio::signal::ctrl_c().await {
            Ok(()) => {
                log::info!("Received shutdown signal (Ctrl+C), shutting down gracefully...");
                cancel_token.cancel();
            }
            Err(err) => {
                log::error!("Failed to listen for shutdown signal: {}", err);
            }
        }
    });

    // Wait for server to finish (either naturally or due to Ctrl+C)
    match service.waiting().await {
        Ok(reason) => {
            log::info!("Server stopped: {:?}", reason);
        }
        Err(e) => {
            log::error!("Server task join error: {:?}", e);
            return Err(io_context("Server task join error", e));
        }
    }

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

    #[test]
    fn test_index_building_respects_config() {
        let config = Config::load().expect("Failed to load config");

        // Verify default configuration
        assert_eq!(config.search.backend, "simple");
        assert_eq!(config.search.rebuild_on_startup, false);
        assert_eq!(config.search.index_path, ".tantivy-index");
    }

    #[tokio::test]
    async fn test_index_path_resolution() {
        let config = Config::load().expect("Failed to load config");
        let index_path = config.search.index_path();

        // Verify path resolution works
        assert!(index_path.is_ok(), "Failed to resolve index path: {:?}", index_path.err());
    }
}
