//! CLI command handling for the Music Theory MCP server.
//!
//! Provides serve/index/status subcommands using clap.

use std::io::Write;

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
    /// Override log level (trace, debug, info, warn, error)
    #[arg(long = "log-level", short = 'l', global = true)]
    pub log_level: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Available CLI commands
#[derive(Subcommand)]
pub enum Commands {
    /// Run the MCP server (default mode)
    Serve {
        /// Test mode: Run without MCP protocol for testing graceful shutdown
        /// In this mode, the server starts but doesn't accept connections.
        /// Use Ctrl+C to test signal handling.
        #[arg(long)]
        test: bool,
    },

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
    let log_level = cli.log_level.clone();

    match cli.command.unwrap_or(Commands::Serve { test: false }) {
        Commands::Serve { test } => run_server(log_level, test).await,

        #[cfg(feature = "fts")]
        Commands::Index { force } => handle_index_command(force, log_level).await,

        #[cfg(feature = "fts")]
        Commands::Status => handle_status_command(log_level).await,
    }
}

/// Apply log level override if provided.
///
/// Since twyg::Opts fields are private, we rebuild the Opts with OptsBuilder
/// using the current values plus the overridden log level.
///
/// # Arguments
///
/// * `opts` - Original twyg options from config
/// * `log_level_override` - Optional log level string to override
///
/// # Returns
///
/// Returns new twyg::Opts with overridden log level, or original if no override.
///
/// # Errors
///
/// Returns `Err` if the log level string is invalid.
fn apply_log_level_override(
    opts: &twyg::Opts,
    log_level_override: Option<String>,
) -> Result<twyg::Opts> {
    if let Some(level_str) = log_level_override {
        use twyg::{LogLevel, OptsBuilder};

        // Parse the log level string
        let level: LogLevel = level_str.parse().map_err(|_| {
            crate::error::Error::config(format!("Invalid log level: {}", level_str))
        })?;

        // Rebuild Opts with all existing values except the level
        OptsBuilder::new()
            .coloured(opts.coloured())
            .output(opts.output().clone())
            .level(level)
            .report_caller(opts.report_caller())
            .timestamp_format(opts.timestamp_format().clone())
            .pad_level(opts.pad_level())
            .pad_amount(opts.pad_amount())
            .pad_side(opts.pad_side())
            .msg_separator(opts.msg_separator())
            .arrow_char(opts.arrow_char())
            .colors(opts.colors().clone())
            .build()
            .map_err(|e| crate::error::Error::config(format!("Failed to build twyg opts: {}", e)))
    } else {
        Ok(opts.clone())
    }
}

/// Run the MCP server.
///
/// This is the default command when no subcommand is specified.
/// Performs the same initialization as the original main() function.
///
/// # Arguments
///
/// * `log_level_override` - Optional log level to override config
/// * `test_mode` - If true, runs in test mode without MCP protocol
async fn run_server(log_level_override: Option<String>, test_mode: bool) -> Result<()> {
    use rmcp::{transport::stdio, ServiceExt};

    // Load configuration
    let config = Config::load()?;

    // Apply log level override if provided
    let log_opts = apply_log_level_override(&config.logging, log_level_override)?;

    // Initialize logging with twyg from config
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

    if test_mode {
        // Test mode: Run without MCP protocol for testing signal handling
        log::info!("Running in TEST MODE (no MCP protocol)");
        log::info!("Server is running. Press Ctrl+C to test graceful shutdown...");

        // Set up signal handler
        let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel();

        tokio::spawn(async move {
            match tokio::signal::ctrl_c().await {
                Ok(()) => {
                    log::info!("Received shutdown signal (Ctrl+C), shutting down gracefully...");
                    let _ = shutdown_tx.send(());
                }
                Err(err) => {
                    log::error!("Failed to listen for shutdown signal: {}", err);
                }
            }
        });

        // Wait for shutdown signal
        let _ = shutdown_rx.await;
        log::info!("Server stopped gracefully");

        return Ok(());
    }

    // Normal mode: Create and run the MCP server with stdio transport
    log::info!(transport = "stdio"; "Starting MCP server");
    log::info!("Waiting for MCP client initialization handshake...");

    let service = MusicTheoryServer::new(state)
        .serve(stdio())
        .await
        .map_err(|e| {
            // Provide helpful error message for common failure cases
            let error_str = format!("{:?}", e);

            if error_str.contains("ConnectionClosed") || error_str.contains("initialized") {
                eprintln!("\n❌ MCP Protocol Error: Failed to complete initialization handshake\n");
                eprintln!("This server uses the Model Context Protocol (MCP) and expects JSON-RPC");
                eprintln!("messages on stdin. It cannot be run interactively in a terminal.\n");
                eprintln!("Usage:");
                eprintln!("  • Run through an MCP client (e.g., Claude Desktop)");
                eprintln!(
                    "  • Use the MCP Inspector for testing: npx @modelcontextprotocol/inspector"
                );
                eprintln!(
                    "  • Use --test flag to test signal handling: music-theory-mcp serve --test"
                );
                eprintln!("  • See: https://modelcontextprotocol.io/docs/tools/inspector\n");
                eprintln!("Original error: {}\n", error_str);

                log::error!(
                    "MCP initialization failed (likely invalid protocol input): {}",
                    error_str
                );
            } else {
                log::error!("Failed to start MCP server: {}", error_str);
            }

            crate::error::Error::io(std::io::Error::other(format!(
                "MCP server initialization failed: {}",
                error_str
            )))
        })?;

    // Set up graceful shutdown handler for Ctrl+C
    let cancel_token = service.cancellation_token();
    tokio::spawn(async move {
        match tokio::signal::ctrl_c().await {
            Ok(()) => {
                log::info!("Received shutdown signal (Ctrl+C), shutting down gracefully...");
                // Force flush to ensure message is written before cancellation
                let _ = std::io::stderr().flush();
                // Small delay to allow log message to be processed
                tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
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
            // Force flush to ensure message is written before exit
            let _ = std::io::stderr().flush();
            // Small delay to allow log message to be processed
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        }
        Err(e) => {
            log::error!("Server task join error: {:?}", e);
            let _ = std::io::stderr().flush();
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
            return Err(crate::error::Error::io(std::io::Error::other(format!(
                "Server task join error: {:?}",
                e
            ))));
        }
    }

    Ok(())
}

/// Setup logging for CLI commands.
///
/// In test mode, logging setup is skipped since the global logger may already be initialized.
/// In production, this properly initializes the logger with the given options.
#[cfg(all(feature = "fts", not(test)))]
fn setup_cli_logging(log_opts: twyg::Opts) -> Result<()> {
    twyg::setup(log_opts)
        .map(|_| ()) // Discard Logger return value
        .map_err(|e| crate::error::Error::config(format!("Failed to setup logging: {}", e)))
}

#[cfg(all(feature = "fts", test))]
fn setup_cli_logging(_log_opts: twyg::Opts) -> Result<()> {
    // In tests, logger may already be initialized - skip to avoid errors
    Ok(())
}

/// Handle the index command (build or rebuild FTS index).
///
/// Builds the index if it doesn't exist or if --force is specified.
/// Skips building if index is fresh and --force is not used.
///
/// # Arguments
///
/// * `force` - Force rebuild even if index is fresh
/// * `log_level_override` - Optional log level to override config
#[cfg(feature = "fts")]
async fn handle_index_command(force: bool, log_level_override: Option<String>) -> Result<()> {
    let config = Config::load()?;

    // Apply log level override if provided
    let log_opts = apply_log_level_override(&config.logging, log_level_override)?;

    // Initialize logging for CLI output
    setup_cli_logging(log_opts)?;

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
///
/// # Arguments
///
/// * `log_level_override` - Optional log level to override config
#[cfg(feature = "fts")]
async fn handle_status_command(log_level_override: Option<String>) -> Result<()> {
    let config = Config::load()?;

    // Apply log level override if provided (for any debug logging)
    let log_opts = apply_log_level_override(&config.logging, log_level_override)?;
    setup_cli_logging(log_opts)?;

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
    #[cfg(feature = "fts")]
    use serial_test::serial;

    #[test]
    fn test_cli_parse_no_command() {
        let cli = Cli::parse_from(["music-theory-mcp"]);
        assert!(cli.command.is_none());
        assert!(cli.log_level.is_none());
    }

    #[test]
    fn test_cli_parse_serve() {
        let cli = Cli::parse_from(["music-theory-mcp", "serve"]);
        assert!(matches!(cli.command, Some(Commands::Serve { test: false })));
        assert!(cli.log_level.is_none());
    }

    #[test]
    fn test_cli_parse_serve_test_mode() {
        let cli = Cli::parse_from(["music-theory-mcp", "serve", "--test"]);
        assert!(matches!(cli.command, Some(Commands::Serve { test: true })));
        assert!(cli.log_level.is_none());
    }

    #[test]
    fn test_cli_parse_log_level_long() {
        let cli = Cli::parse_from(["music-theory-mcp", "--log-level", "debug"]);
        assert_eq!(cli.log_level, Some("debug".to_string()));
    }

    #[test]
    fn test_cli_parse_log_level_short() {
        let cli = Cli::parse_from(["music-theory-mcp", "-l", "trace"]);
        assert_eq!(cli.log_level, Some("trace".to_string()));
    }

    #[test]
    fn test_cli_parse_log_level_with_serve() {
        let cli = Cli::parse_from(["music-theory-mcp", "--log-level", "warn", "serve"]);
        assert!(matches!(cli.command, Some(Commands::Serve { test: false })));
        assert_eq!(cli.log_level, Some("warn".to_string()));
    }

    #[test]
    fn test_cli_parse_log_level_before_command() {
        let cli = Cli::parse_from(["music-theory-mcp", "-l", "error", "serve"]);
        assert!(matches!(cli.command, Some(Commands::Serve { test: false })));
        assert_eq!(cli.log_level, Some("error".to_string()));
    }

    #[test]
    fn test_cli_parse_log_level_after_command() {
        let cli = Cli::parse_from(["music-theory-mcp", "serve", "--log-level", "info"]);
        assert!(matches!(cli.command, Some(Commands::Serve { test: false })));
        assert_eq!(cli.log_level, Some("info".to_string()));
    }

    #[test]
    fn test_cli_parse_serve_test_with_log_level() {
        let cli = Cli::parse_from(["music-theory-mcp", "-l", "debug", "serve", "--test"]);
        assert!(matches!(cli.command, Some(Commands::Serve { test: true })));
        assert_eq!(cli.log_level, Some("debug".to_string()));
    }

    #[test]
    #[cfg(feature = "fts")]
    fn test_cli_parse_index() {
        let cli = Cli::parse_from(&["music-theory-mcp", "index"]);
        assert!(matches!(
            cli.command,
            Some(Commands::Index { force: false })
        ));
        assert!(cli.log_level.is_none());
    }

    #[test]
    #[cfg(feature = "fts")]
    fn test_cli_parse_index_force() {
        let cli = Cli::parse_from(&["music-theory-mcp", "index", "--force"]);
        assert!(matches!(cli.command, Some(Commands::Index { force: true })));
        assert!(cli.log_level.is_none());
    }

    #[test]
    #[cfg(feature = "fts")]
    fn test_cli_parse_index_force_short() {
        let cli = Cli::parse_from(&["music-theory-mcp", "index", "-f"]);
        assert!(matches!(cli.command, Some(Commands::Index { force: true })));
        assert!(cli.log_level.is_none());
    }

    #[test]
    #[cfg(feature = "fts")]
    fn test_cli_parse_index_with_log_level() {
        let cli = Cli::parse_from(&["music-theory-mcp", "-l", "debug", "index", "--force"]);
        assert!(matches!(cli.command, Some(Commands::Index { force: true })));
        assert_eq!(cli.log_level, Some("debug".to_string()));
    }

    #[test]
    #[cfg(feature = "fts")]
    fn test_cli_parse_status() {
        let cli = Cli::parse_from(&["music-theory-mcp", "status"]);
        assert!(matches!(cli.command, Some(Commands::Status)));
        assert!(cli.log_level.is_none());
    }

    #[test]
    #[cfg(feature = "fts")]
    fn test_cli_parse_status_with_log_level() {
        let cli = Cli::parse_from(&["music-theory-mcp", "--log-level", "trace", "status"]);
        assert!(matches!(cli.command, Some(Commands::Status)));
        assert_eq!(cli.log_level, Some("trace".to_string()));
    }

    #[test]
    fn test_apply_log_level_override_none() {
        use twyg::{LogLevel, OptsBuilder, Output};

        let original = OptsBuilder::new()
            .level(LogLevel::Info)
            .coloured(true)
            .output(Output::Stderr)
            .build()
            .unwrap();

        let result = apply_log_level_override(&original, None).unwrap();
        assert_eq!(result.level(), LogLevel::Info);
    }

    #[test]
    fn test_apply_log_level_override_some() {
        use twyg::{LogLevel, OptsBuilder, Output};

        let original = OptsBuilder::new()
            .level(LogLevel::Info)
            .coloured(true)
            .output(Output::Stderr)
            .build()
            .unwrap();

        let result = apply_log_level_override(&original, Some("debug".to_string())).unwrap();
        assert_eq!(result.level(), LogLevel::Debug);
        // Verify other fields preserved
        assert_eq!(result.coloured(), original.coloured());
        assert_eq!(result.output(), original.output());
    }

    #[test]
    fn test_apply_log_level_override_invalid() {
        use twyg::{LogLevel, OptsBuilder, Output};

        let original = OptsBuilder::new()
            .level(LogLevel::Info)
            .coloured(true)
            .output(Output::Stderr)
            .build()
            .unwrap();

        let result = apply_log_level_override(&original, Some("invalid".to_string()));
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Invalid log level"));
    }

    #[test]
    fn test_apply_log_level_override_all_levels() {
        use twyg::{LogLevel, OptsBuilder, Output};

        let original = OptsBuilder::new()
            .level(LogLevel::Info)
            .coloured(true)
            .output(Output::Stderr)
            .build()
            .unwrap();

        let levels = vec![
            ("trace", LogLevel::Trace),
            ("debug", LogLevel::Debug),
            ("info", LogLevel::Info),
            ("warn", LogLevel::Warn),
            ("error", LogLevel::Error),
        ];

        for (level_str, expected_level) in levels {
            let result = apply_log_level_override(&original, Some(level_str.to_string())).unwrap();
            assert_eq!(
                result.level(),
                expected_level,
                "Failed for level: {}",
                level_str
            );
        }
    }

    #[tokio::test]
    #[ignore = "Hangs waiting for MCP protocol on stdin - needs mocking"]
    async fn test_handle_command_serve_default() {
        // Test that handle_command dispatches to serve when no command given
        let cli = Cli {
            log_level: None,
            command: None,
        };

        // This will try to run the server, which will hang waiting for stdin
        // TODO: Mock stdio transport to test this properly
        let result = handle_command(cli).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    #[serial]
    #[cfg(feature = "fts")]
    async fn test_handle_index_command_no_index() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let concept_cards_path = temp_dir.path().join("concept-cards");
        fs::create_dir_all(&concept_cards_path).expect("Failed to create concept cards dir");

        // Create a test concept card
        let card_content = r#"---
title: Test Card
category: test
---

# Test Card

Test content.
"#;
        fs::write(concept_cards_path.join("test.md"), card_content)
            .expect("Failed to write test card");

        // Create a minimal config file
        let config_content = format!(
            r#"
[server]
name = "test-server"
version = "0.1.0"

[paths]
base = "{}"
sources_md = "sources-md"
concept_cards = "{}"
concepts_unified = "concepts-unified"
guides = "guides"
skill_docs = "."

[sources]

[logging]
level = "error"
coloured = false
output = "stderr"
report_caller = false

[search]
backend = "tantivy"
index_path = "{}"
rebuild_on_startup = false
snippet_size = 200
fuzzy_search = false
fuzzy_distance = 2
"#,
            temp_dir.path().display(),
            concept_cards_path.display(),
            temp_dir.path().join("test-index").display()
        );

        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(&config_dir).expect("Failed to create config dir");
        let config_path = config_dir.join("default.toml");
        fs::write(&config_path, config_content).expect("Failed to write config");

        // Set env var to use our test config
        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", config_dir);

        // Run index command (force=false, should build since no index exists)
        let result = handle_index_command(false, None).await;

        // Clean up env var
        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        if let Err(ref e) = result {
            eprintln!("Error: {:?}", e);
        }
        assert!(result.is_ok(), "Failed with error: {:?}", result.err());
    }

    #[tokio::test]
    #[serial]
    #[cfg(feature = "fts")]
    async fn test_handle_index_command_with_fresh_index() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let index_path = temp_dir.path().join("test-index");
        let concept_cards_path = temp_dir.path().join("concept-cards");
        fs::create_dir_all(&concept_cards_path).expect("Failed to create concept cards dir");

        let card_content = r#"---
title: Test Card
category: test
---

# Test Card

Test content.
"#;
        fs::write(concept_cards_path.join("test.md"), card_content)
            .expect("Failed to write test card");

        let config_content = format!(
            r#"
[server]
name = "test-server"
version = "0.1.0"

[paths]
base = "{}"
sources_md = "sources-md"
concept_cards = "{}"
concepts_unified = "concepts-unified"
guides = "guides"
skill_docs = "."

[sources]

[logging]
level = "error"
coloured = false
output = "stderr"
report_caller = false

[search]
backend = "tantivy"
index_path = "{}"
rebuild_on_startup = false
snippet_size = 200
fuzzy_search = false
fuzzy_distance = 2
"#,
            temp_dir.path().display(),
            concept_cards_path.display(),
            index_path.display()
        );

        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(&config_dir).expect("Failed to create config dir");
        fs::write(config_dir.join("default.toml"), config_content).expect("Failed to write config");

        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        // Build index first
        build_index(&Config::load().unwrap())
            .await
            .expect("Failed to build index");

        // Now try to index again without force - should skip
        let result = handle_index_command(false, None).await;

        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(result.is_ok());
    }

    #[tokio::test]
    #[serial]
    #[cfg(feature = "fts")]
    async fn test_handle_index_command_with_force() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let index_path = temp_dir.path().join("test-index");
        let concept_cards_path = temp_dir.path().join("concept-cards");
        fs::create_dir_all(&concept_cards_path).expect("Failed to create concept cards dir");

        let card_content = r#"---
title: Test Card
category: test
---

# Test Card

Test content.
"#;
        fs::write(concept_cards_path.join("test.md"), card_content)
            .expect("Failed to write test card");

        let config_content = format!(
            r#"
[server]
name = "test-server"
version = "0.1.0"

[paths]
base = "{}"
sources_md = "sources-md"
concept_cards = "{}"
concepts_unified = "concepts-unified"
guides = "guides"
skill_docs = "."

[sources]

[logging]
level = "error"
coloured = false
output = "stderr"
report_caller = false

[search]
backend = "tantivy"
index_path = "{}"
rebuild_on_startup = false
snippet_size = 200
fuzzy_search = false
fuzzy_distance = 2
"#,
            temp_dir.path().display(),
            concept_cards_path.display(),
            index_path.display()
        );

        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(&config_dir).expect("Failed to create config dir");
        fs::write(config_dir.join("default.toml"), config_content).expect("Failed to write config");

        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        // Build with force=true (should always build)
        let result = handle_index_command(true, None).await;

        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        if let Err(ref e) = result {
            eprintln!("test_handle_index_command_with_force error: {}", e);
        }
        assert!(result.is_ok(), "Expected Ok, got: {:?}", result.err());
    }

    #[tokio::test]
    #[serial]
    #[cfg(feature = "fts")]
    async fn test_handle_status_command_no_index() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let config_content = format!(
            r#"
[server]
name = "test-server"
version = "0.1.0"

[paths]
base = "{}"
sources_md = "sources-md"
concept_cards = "concept-cards"
concepts_unified = "concepts-unified"
guides = "guides"
skill_docs = "."

[sources]

[logging]
level = "error"
coloured = false
output = "stderr"
report_caller = false

[search]
backend = "tantivy"
index_path = "{}"
rebuild_on_startup = false
snippet_size = 200
fuzzy_search = false
fuzzy_distance = 2
"#,
            temp_dir.path().display(),
            temp_dir.path().join("nonexistent-index").display()
        );

        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(&config_dir).expect("Failed to create config dir");
        fs::write(config_dir.join("default.toml"), config_content).expect("Failed to write config");

        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        // Should succeed but report no index
        let result = handle_status_command(None).await;

        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(result.is_ok());
    }

    #[tokio::test]
    #[serial]
    #[cfg(feature = "fts")]
    async fn test_handle_status_command_no_metadata() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let index_path = temp_dir.path().join("test-index");
        fs::create_dir_all(&index_path).expect("Failed to create index dir");

        let config_content = format!(
            r#"
[server]
name = "test-server"
version = "0.1.0"

[paths]
base = "{}"
sources_md = "sources-md"
concept_cards = "concept-cards"
concepts_unified = "concepts-unified"
guides = "guides"
skill_docs = "."

[sources]

[logging]
level = "error"
coloured = false
output = "stderr"
report_caller = false

[search]
backend = "tantivy"
index_path = "{}"
rebuild_on_startup = false
snippet_size = 200
fuzzy_search = false
fuzzy_distance = 2
"#,
            temp_dir.path().display(),
            index_path.display()
        );

        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(&config_dir).expect("Failed to create config dir");
        fs::write(config_dir.join("default.toml"), config_content).expect("Failed to write config");

        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        // Should succeed but report no metadata
        let result = handle_status_command(None).await;

        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(result.is_ok());
    }

    #[tokio::test]
    #[serial]
    #[cfg(feature = "fts")]
    async fn test_handle_status_command_with_index() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let index_path = temp_dir.path().join("test-index");
        let concept_cards_path = temp_dir.path().join("concept-cards");
        fs::create_dir_all(&concept_cards_path).expect("Failed to create concept cards dir");

        let card_content = r#"---
title: Test Card
category: test
---

# Test Card

Test content.
"#;
        fs::write(concept_cards_path.join("test.md"), card_content)
            .expect("Failed to write test card");

        let config_content = format!(
            r#"
[server]
name = "test-server"
version = "0.1.0"

[paths]
base = "{}"
sources_md = "sources-md"
concept_cards = "{}"
concepts_unified = "concepts-unified"
guides = "guides"
skill_docs = "."

[sources]

[logging]
level = "error"
coloured = false
output = "stderr"
report_caller = false

[search]
backend = "tantivy"
index_path = "{}"
rebuild_on_startup = false
snippet_size = 200
fuzzy_search = false
fuzzy_distance = 2
"#,
            temp_dir.path().display(),
            concept_cards_path.display(),
            index_path.display()
        );

        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(&config_dir).expect("Failed to create config dir");
        fs::write(config_dir.join("default.toml"), config_content).expect("Failed to write config");

        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        // Build index first
        build_index(&Config::load().unwrap())
            .await
            .expect("Failed to build index");

        // Now check status
        let result = handle_status_command(None).await;

        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(result.is_ok());
    }

    #[tokio::test]
    #[serial]
    #[cfg(feature = "fts")]
    async fn test_handle_status_command_with_stale_index() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let index_path = temp_dir.path().join("test-index");
        let concept_cards_path = temp_dir.path().join("concept-cards");
        fs::create_dir_all(&concept_cards_path).expect("Failed to create concept cards dir");

        let card_content = r#"---
title: Test Card
category: test
---

# Test Card

Test content.
"#;
        fs::write(concept_cards_path.join("test.md"), card_content)
            .expect("Failed to write test card");

        let config_content = format!(
            r#"
[server]
name = "test-server"
version = "0.1.0"

[paths]
base = "{}"
sources_md = "sources-md"
concept_cards = "{}"
concepts_unified = "concepts-unified"
guides = "guides"
skill_docs = "."

[sources]

[logging]
level = "error"
coloured = false
output = "stderr"
report_caller = false

[search]
backend = "tantivy"
index_path = "{}"
rebuild_on_startup = false
snippet_size = 200
fuzzy_search = false
fuzzy_distance = 2
"#,
            temp_dir.path().display(),
            concept_cards_path.display(),
            index_path.display()
        );

        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(&config_dir).expect("Failed to create config dir");
        fs::write(config_dir.join("default.toml"), config_content).expect("Failed to write config");

        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        // Build index
        build_index(&Config::load().unwrap())
            .await
            .expect("Failed to build index");

        // Add a new file to make index stale
        fs::write(concept_cards_path.join("new.md"), card_content)
            .expect("Failed to write new card");

        // Check status - should report stale
        let result = handle_status_command(None).await;

        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(result.is_ok());
    }

    #[tokio::test]
    #[serial]
    #[cfg(feature = "fts")]
    async fn test_handle_index_command_with_log_level() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let concept_cards_path = temp_dir.path().join("concept-cards");
        fs::create_dir_all(&concept_cards_path).expect("Failed to create concept cards dir");

        let card_content = r#"---
title: Test Card
category: test
---

# Test Card

Test content.
"#;
        fs::write(concept_cards_path.join("test.md"), card_content)
            .expect("Failed to write test card");

        let config_content = format!(
            r#"
[server]
name = "test-server"
version = "0.1.0"

[paths]
base = "{}"
sources_md = "sources-md"
concept_cards = "{}"
concepts_unified = "concepts-unified"
guides = "guides"
skill_docs = "."

[sources]

[logging]
level = "error"
coloured = false
output = "stderr"
report_caller = false

[search]
backend = "tantivy"
index_path = "{}"
rebuild_on_startup = false
snippet_size = 200
fuzzy_search = false
fuzzy_distance = 2
"#,
            temp_dir.path().display(),
            concept_cards_path.display(),
            temp_dir.path().join("test-index").display()
        );

        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(&config_dir).expect("Failed to create config dir");
        fs::write(config_dir.join("default.toml"), config_content).expect("Failed to write config");

        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        // Test with log level override
        let result = handle_index_command(false, Some("debug".to_string())).await;

        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(result.is_ok());
    }

    #[tokio::test]
    #[serial]
    #[cfg(feature = "fts")]
    async fn test_handle_status_command_with_log_level() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let config_content = format!(
            r#"
[server]
name = "test-server"
version = "0.1.0"

[paths]
base = "{}"
sources_md = "sources-md"
concept_cards = "concept-cards"
concepts_unified = "concepts-unified"
guides = "guides"
skill_docs = "."

[sources]

[logging]
level = "error"
coloured = false
output = "stderr"
report_caller = false

[search]
backend = "tantivy"
index_path = "{}"
rebuild_on_startup = false
snippet_size = 200
fuzzy_search = false
fuzzy_distance = 2
"#,
            temp_dir.path().display(),
            temp_dir.path().join("nonexistent").display()
        );

        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(&config_dir).expect("Failed to create config dir");
        fs::write(config_dir.join("default.toml"), config_content).expect("Failed to write config");

        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        // Test with log level override
        let result = handle_status_command(Some("trace".to_string())).await;

        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(result.is_ok());
    }
}
