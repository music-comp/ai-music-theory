//! CLI command handling for the Music Theory MCP server.
//!
//! Provides serve/index/status subcommands using clap.

#[cfg(feature = "fts")]
use std::sync::Arc;

use clap::{Parser, Subcommand};

use crate::config::Config;
use crate::error::Result;
use crate::server::MusicTheoryServer;
use crate::state::AppState;

#[cfg(feature = "fts")]
use crate::search::{build_index, is_index_fresh, IndexMetadata};

/// Music Theory AI Skill MCP Server
#[derive(Parser)]
#[command(
    name = "music-theory-mcp",
    version,
    about = "Music Theory AI Skill MCP Server",
    long_about = "MCP server providing access to music theory educational materials \
                  including source texts, concept cards, and topic guides."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Available CLI commands
#[derive(Subcommand)]
pub enum Commands {
    /// Run the MCP server (default mode)
    Serve,

    /// Build or rebuild the full-text search index
    #[cfg(feature = "fts")]
    Index {
        /// Force rebuild even if index is current
        #[arg(long, short)]
        force: bool,
    },

    /// Display FTS index status and statistics
    #[cfg(feature = "fts")]
    Status,
}

/// Handle the CLI command.
///
/// Dispatches to the appropriate handler based on the command.
/// If no command is provided, defaults to Serve.
///
/// # Arguments
///
/// * `cli` - Parsed CLI arguments
///
/// # Errors
///
/// Returns `Err` if command execution fails.
pub async fn handle_command(cli: Cli) -> Result<()> {
    match cli.command.unwrap_or(Commands::Serve) {
        Commands::Serve => run_server().await,

        #[cfg(feature = "fts")]
        Commands::Index { force } => handle_index_command(force).await,

        #[cfg(feature = "fts")]
        Commands::Status => handle_status_command().await,
    }
}

/// Run the MCP server.
///
/// This is the default command when no subcommand is specified.
/// Performs the same initialization as the original main() function.
async fn run_server() -> Result<()> {
    use rmcp::{transport::stdio, ServiceExt};

    // Load configuration
    let config = Config::load()?;

    // Initialize logging with twyg from config
    let log_opts = config.logging.to_twyg()?;
    twyg::setup(log_opts)
        .map_err(|e| crate::error::Error::config(format!("Failed to setup logging: {}", e)))?;

    log::info!(
        version = &*config.server.version,
        name = &*config.server.name;
        "Music Theory MCP Server starting"
    );

    // Create application state
    log::info!("Creating application state");
    let state = AppState::new(config).await?;

    // Initialize FTS (non-blocking - may start background indexing)
    #[cfg(feature = "fts")]
    {
        let state_arc = Arc::new(state.clone());
        crate::state::initialize_fts(&state_arc).await?;
    }

    // Create and run the MCP server with stdio transport
    log::info!(transport = "stdio"; "Starting MCP server");
    let service = MusicTheoryServer::new(state)
        .serve(stdio())
        .await
        .map_err(|e| {
            crate::error::Error::io(std::io::Error::other(format!(
                "Failed to start server: {:?}",
                e
            )))
        })?;

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
            return Err(crate::error::Error::io(std::io::Error::other(format!(
                "Server task join error: {:?}",
                e
            ))));
        }
    }

    Ok(())
}

/// Handle the index command (build or rebuild FTS index).
///
/// Builds the index if it doesn't exist or if --force is specified.
/// Skips building if index is fresh and --force is not used.
#[cfg(feature = "fts")]
async fn handle_index_command(force: bool) -> Result<()> {
    let config = Config::load()?;

    // Initialize logging for CLI output
    let log_opts = config.logging.to_twyg()?;
    twyg::setup(log_opts)
        .map_err(|e| crate::error::Error::config(format!("Failed to setup logging: {}", e)))?;

    let index_path = config.search.index_path()?;

    // Check if index is fresh (unless force rebuild)
    if !force && is_index_fresh(&index_path, &config).await? {
        println!("✓ Index is up to date.");
        println!("  Use --force to rebuild anyway.");
        return Ok(());
    }

    // Build index
    println!("Building FTS index...");
    let stats = build_index(&config).await?;

    println!("✓ Index build complete:");
    println!("  Files found:      {}", stats.files_found);
    println!("  Documents indexed: {}", stats.indexed);
    println!("  Errors:           {}", stats.errors);
    println!("  Location:         {}", index_path.display());

    Ok(())
}

/// Handle the status command (show FTS index information).
///
/// Displays index metadata including document count, last indexed time,
/// and freshness status.
#[cfg(feature = "fts")]
async fn handle_status_command() -> Result<()> {
    let config = Config::load()?;
    let index_path = config.search.index_path()?;

    if !index_path.exists() {
        println!("✗ No index found");
        println!("  Location: {}", index_path.display());
        println!("  Run 'music-theory-mcp index' to build one.");
        return Ok(());
    }

    // Load metadata
    let metadata_path = index_path.join("metadata.json");
    if !metadata_path.exists() {
        println!("✓ Index exists but no metadata found");
        println!("  Location: {}", index_path.display());
        println!("  This may be an old index. Run 'music-theory-mcp index --force' to rebuild.");
        return Ok(());
    }

    let json = tokio::fs::read_to_string(&metadata_path).await?;
    let metadata: IndexMetadata = serde_json::from_str(&json)?;

    // Check freshness
    let is_fresh = is_index_fresh(&index_path, &config).await?;

    println!("Index Status:");
    println!("  Location:     {}", index_path.display());
    println!("  Documents:    {}", metadata.doc_count);
    println!(
        "  Last indexed: {:?}",
        metadata
            .last_indexed
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .map(|d| format!("{} seconds ago", d.as_secs()))
            .unwrap_or_else(|_| "unknown".to_string())
    );
    println!(
        "  Status:       {}",
        if is_fresh {
            "✓ Current"
        } else {
            "✗ Stale (rebuild recommended)"
        }
    );

    if !is_fresh {
        println!("\nRun 'music-theory-mcp index' to rebuild.");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parse_no_command() {
        let cli = Cli::parse_from(&["music-theory-mcp"]);
        assert!(cli.command.is_none());
    }

    #[test]
    fn test_cli_parse_serve() {
        let cli = Cli::parse_from(&["music-theory-mcp", "serve"]);
        assert!(matches!(cli.command, Some(Commands::Serve)));
    }

    #[test]
    #[cfg(feature = "fts")]
    fn test_cli_parse_index() {
        let cli = Cli::parse_from(&["music-theory-mcp", "index"]);
        assert!(matches!(cli.command, Some(Commands::Index { force: false })));
    }

    #[test]
    #[cfg(feature = "fts")]
    fn test_cli_parse_index_force() {
        let cli = Cli::parse_from(&["music-theory-mcp", "index", "--force"]);
        assert!(matches!(cli.command, Some(Commands::Index { force: true })));
    }

    #[test]
    #[cfg(feature = "fts")]
    fn test_cli_parse_index_force_short() {
        let cli = Cli::parse_from(&["music-theory-mcp", "index", "-f"]);
        assert!(matches!(cli.command, Some(Commands::Index { force: true })));
    }

    #[test]
    #[cfg(feature = "fts")]
    fn test_cli_parse_status() {
        let cli = Cli::parse_from(&["music-theory-mcp", "status"]);
        assert!(matches!(cli.command, Some(Commands::Status)));
    }
}
