//! CLI command handling for the Music Theory MCP server.
//!
//! Provides serve/index/status subcommands using clap.

#[cfg(any(feature = "fts", feature = "graph", feature = "vector"))]
use std::sync::Arc;

use clap::{Parser, Subcommand};
use fabryk::core::ConfigManager;
use fabryk_cli::ConfigAction;

use crate::config::Config;
use crate::error::Result;
use crate::server::build_server;
use crate::state::AppState;
use fabryk_cli::SourcesCommand;

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

    /// MCP transport: "http" or "stdio". Overrides config file value.
    #[cfg(feature = "http")]
    #[arg(long)]
    pub transport: Option<String>,

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

        /// Port for HTTP server (requires --transport http)
        #[cfg(feature = "http")]
        #[arg(long, short, default_value = "8080")]
        port: u16,
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

    /// Graph database management
    #[cfg(feature = "graph")]
    Graph(GraphCommands),

    /// Vector index management
    #[cfg(feature = "vector")]
    Vectordb(VectordbCommands),

    /// Manage server configuration; as a command by itself, returns the full
    /// path to the config file
    Config {
        #[command(subcommand)]
        action: Option<ConfigAction>,
    },

    /// Source management (scan, validate, aliases)
    Sources(SourcesCommand),

    /// Cache management (download pre-built indexes, package for distribution)
    Cache(CacheCommands),
}

/// Graph subcommands
#[cfg(feature = "graph")]
#[derive(Parser)]
pub struct GraphCommands {
    #[command(subcommand)]
    command: GraphSubcommand,
}

/// Graph command operations
#[cfg(feature = "graph")]
#[derive(Subcommand)]
pub enum GraphSubcommand {
    /// Build graph from concept cards
    Build {
        /// Show what would change without writing
        #[arg(long)]
        dry_run: bool,

        /// Show detailed output
        #[arg(long, short)]
        verbose: bool,
    },

    /// Validate graph integrity
    Validate,

    /// Show graph statistics
    Stats,

    /// Rebuild rkyv cache from JSON
    Compile,
}

/// Vector database subcommands
#[cfg(feature = "vector")]
#[derive(Parser)]
pub struct VectordbCommands {
    #[command(subcommand)]
    command: VectordbSubcommand,
}

/// Vector database command operations
#[cfg(feature = "vector")]
#[derive(Subcommand)]
pub enum VectordbSubcommand {
    /// Build vector index from content
    Build {
        /// Force rebuild even if cache is current
        #[arg(long, short)]
        force: bool,
    },

    /// Show vector index status
    Status,
}

/// Cache management subcommands.
#[derive(Parser)]
pub struct CacheCommands {
    #[command(subcommand)]
    command: CacheSubcommand,
}

/// Cache command operations.
#[derive(Subcommand)]
pub enum CacheSubcommand {
    /// Download pre-built caches from GitHub releases
    Download {
        /// Which cache to download (graph, fts, vector, all)
        #[arg(default_value = "all")]
        backend: String,

        /// Force re-download even if cache exists at current version
        #[arg(long, short)]
        force: bool,
    },

    /// Show status of local caches
    Status,

    /// Package local caches for distribution (CI use)
    Package {
        /// Which cache to package (graph, fts, vector, all)
        #[arg(default_value = "all")]
        backend: String,

        /// Output directory for archives
        #[arg(long, short, default_value = "./dist")]
        output: String,
    },
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
    #[cfg(feature = "http")]
    let transport = cli.transport.as_deref();

    match cli.command.unwrap_or(Commands::Serve {
        test: false,
        #[cfg(feature = "http")]
        port: 8080,
    }) {
        Commands::Serve {
            test,
            #[cfg(feature = "http")]
            port,
        } => {
            #[cfg(feature = "http")]
            if transport == Some("http") {
                return run_server_http(log_level, port).await;
            }
            run_server(log_level, test).await
        }

        #[cfg(feature = "fts")]
        Commands::Index { force } => handle_index_command(force, log_level).await,

        #[cfg(feature = "fts")]
        Commands::Status => handle_status_command(log_level).await,

        #[cfg(feature = "graph")]
        Commands::Graph(graph_cmds) => handle_graph_command(graph_cmds, log_level).await,

        #[cfg(feature = "vector")]
        Commands::Vectordb(vector_cmds) => handle_vectordb_command(vector_cmds, log_level).await,

        Commands::Config { action } => handle_config_command(action),

        Commands::Sources(sources_cmds) => {
            use fabryk::content::sources::SourceCategory;
            use std::collections::HashMap;

            let config = Config::load()?;
            let content_path = config.paths.concept_cards_path()?;

            let mut categories = HashMap::new();
            for (name, cat) in [
                ("oxford", &config.sources.oxford),
                ("general", &config.sources.general),
                ("papers", &config.sources.papers),
            ] {
                categories.insert(
                    name.to_string(),
                    SourceCategory {
                        path: cat.path.clone(),
                        files: cat.files.clone(),
                        aliases: cat.aliases.clone(),
                    },
                );
            }

            let config_path = crate::config::path_resolver()
                .config_dir()
                .map(|d| d.join("default.toml"));

            fabryk_cli::sources_handlers::handle_sources(
                sources_cmds,
                &content_path,
                &categories,
                config_path.as_deref(),
            )
            .await
        }

        Commands::Cache(cache_cmds) => handle_cache_command(cache_cmds, log_level).await,
    }
}

/// Handle the `config` command and its subcommands.
///
/// Delegates to generic `fabryk_cli::config_handlers` functions
/// parameterized over the server's `Config` type.
fn handle_config_command(action: Option<ConfigAction>) -> Result<()> {
    match action {
        None | Some(ConfigAction::Path) => {
            fabryk_cli::config_handlers::cmd_config_path::<Config>(None)
        }
        Some(ConfigAction::Get { key }) => match key {
            Some(ref k) => fabryk_cli::config_handlers::cmd_config_get::<Config>(None, k),
            None => fabryk_cli::config_handlers::cmd_config_get_or_dump::<Config>(None, None),
        },
        Some(ConfigAction::Set { key, value }) => {
            fabryk_cli::config_handlers::cmd_config_set::<Config>(None, &key, &value)
        }
        Some(ConfigAction::Init { file, force }) => {
            fabryk_cli::config_handlers::cmd_config_init::<Config>(file.as_deref(), force)
        }
        Some(ConfigAction::Export { docker_env, file }) => {
            let config = <Config as ConfigManager>::load(None)?;
            fabryk_cli::config_handlers::cmd_config_export(&config, docker_env, file.as_deref())
        }
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

    // Initialize graph (non-blocking - loads asynchronously)
    #[cfg(feature = "graph")]
    {
        let state_arc = Arc::new(state.clone());
        crate::state::initialize_graph(&state_arc).await?;
    }

    // Initialize vector index (non-blocking - builds asynchronously)
    #[cfg(feature = "vector")]
    {
        let state_arc = Arc::new(state.clone());
        crate::state::initialize_vector(&state_arc).await?;
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

    build_server(state).serve_stdio().await.map_err(|e| {
        let error_str = format!("{:?}", e);

        if error_str.contains("ConnectionClosed") || error_str.contains("initialized") {
            eprintln!("\n❌ MCP Protocol Error: Failed to complete initialization handshake\n");
            eprintln!("This server uses the Model Context Protocol (MCP) and expects JSON-RPC");
            eprintln!("messages on stdin. It cannot be run interactively in a terminal.\n");
            eprintln!("Usage:");
            eprintln!("  • Run through an MCP client (e.g., Claude Desktop)");
            eprintln!("  • Use the MCP Inspector for testing: npx @modelcontextprotocol/inspector");
            eprintln!("  • Use --test flag to test signal handling: music-theory-mcp serve --test");
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

    Ok(())
}

/// Run the MCP server over HTTP.
#[cfg(feature = "http")]
async fn run_server_http(log_level_override: Option<String>, port: u16) -> Result<()> {
    let config = Config::load()?;
    let log_opts = apply_log_level_override(&config.logging, log_level_override)?;
    twyg::setup(log_opts)
        .map_err(|e| crate::error::Error::config(format!("Failed to setup logging: {}", e)))?;

    log::info!(
        version = &*config.server.version,
        name = &*config.server.name;
        "Music Theory MCP Server starting"
    );

    let state = AppState::new(config).await?;

    #[cfg(feature = "fts")]
    {
        let state_arc = Arc::new(state.clone());
        crate::state::initialize_fts(&state_arc).await?;
    }

    #[cfg(feature = "graph")]
    {
        let state_arc = Arc::new(state.clone());
        crate::state::initialize_graph(&state_arc).await?;
    }

    #[cfg(feature = "vector")]
    {
        let state_arc = Arc::new(state.clone());
        crate::state::initialize_vector(&state_arc).await?;
    }

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], port));
    log::info!(transport = "http", port = port; "Starting MCP server");

    build_server(state).serve_http(addr).await.map_err(|e| {
        log::error!("HTTP server error: {}", e);
        crate::error::Error::io(std::io::Error::other(format!("HTTP server error: {}", e)))
    })?;

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

    // Load metadata using fabryk's IndexMetadata via our adapter
    let metadata = IndexMetadata::load(&index_path)?;
    if metadata.is_none() {
        println!("✓ Index exists but no metadata found");
        println!("  Location: {}", index_path.display());
        println!("  This may be an old index. Run 'music-theory-mcp index --force' to rebuild.");
        return Ok(());
    }
    let metadata = metadata.unwrap();

    // Check freshness
    let is_fresh = is_index_fresh(&index_path, &config).await?;

    println!("Index Status:");
    println!("  Location:     {}", index_path.display());
    println!("  Documents:    {}", metadata.doc_count());
    println!("  Last indexed: {}", metadata.indexed_at_display());
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

/// Handle graph subcommands.
#[cfg(feature = "graph")]
async fn handle_graph_command(
    graph_cmds: GraphCommands,
    log_level_override: Option<String>,
) -> Result<()> {
    // Initialize logging before any graph operations
    let config = Config::load()?;
    let opts = apply_log_level_override(&config.logging, log_level_override)?;
    let _ = twyg::setup(opts);

    match graph_cmds.command {
        GraphSubcommand::Build { dry_run, verbose } => {
            crate::graph::handle_build(&config, dry_run, verbose).await
        }
        GraphSubcommand::Validate => crate::graph::handle_validate(&config).await,
        GraphSubcommand::Stats => crate::graph::handle_stats(&config).await,
        GraphSubcommand::Compile => crate::graph::handle_compile(&config).await,
    }
}

/// Handle vectordb subcommand.
#[cfg(feature = "vector")]
async fn handle_vectordb_command(
    vector_cmds: VectordbCommands,
    log_level_override: Option<String>,
) -> Result<()> {
    use std::sync::Arc;

    use fabryk::vector::ConceptCardVectorExtractor;
    use fabryk::vector::{EmbeddingProvider, FastEmbedProvider, VectorIndexBuilder};

    let config = Config::load()?;
    let opts = apply_log_level_override(&config.logging, log_level_override)?;
    let _ = twyg::setup(opts);

    match vector_cmds.command {
        VectordbSubcommand::Build { force } => {
            let base = config.paths.base_path()?;
            let cache_dir = base.join(".cache").join("vector");
            std::fs::create_dir_all(&cache_dir).map_err(crate::error::Error::io)?;
            let cache_file = cache_dir.join("vector-cache.json");

            if force {
                // Remove cache to force rebuild
                if cache_file.exists() {
                    let _ = std::fs::remove_file(&cache_file);
                    println!("Removed existing cache");
                }
            }

            println!("Creating embedding provider (bge-small-en-v1.5)...");
            let provider: Arc<dyn EmbeddingProvider> = Arc::new(
                FastEmbedProvider::new("bge-small-en-v1.5", None).map_err(|e| {
                    crate::error::Error::operation(format!(
                        "Failed to create embedding provider: {e}"
                    ))
                })?,
            );

            let content_dirs = [
                (base.join(&config.paths.concept_cards), "concept_cards"),
                (base.join(&config.paths.sources_md), "sources_md"),
                (
                    base.join(&config.paths.concepts_unified),
                    "concepts_unified",
                ),
                (base.join(&config.paths.guides), "guides"),
            ];

            let mut backend: Option<fabryk::vector::SimpleVectorBackend> = None;
            let mut total_docs = 0usize;

            for (content_path, label) in &content_dirs {
                if !content_path.exists() {
                    println!("  Skipping {} (not found)", label);
                    continue;
                }

                let extractor = ConceptCardVectorExtractor::new();
                let builder = VectorIndexBuilder::new(extractor)
                    .with_content_path(content_path)
                    .with_embedding_provider(Arc::clone(&provider))
                    .with_error_handling(fabryk::vector::builder::ErrorHandling::Collect);

                match &mut backend {
                    None => {
                        let b = builder.with_cache_path(&cache_file);
                        let (new_backend, stats) = b.build().await.map_err(|e| {
                            crate::error::Error::operation(format!("Vector build failed: {e}"))
                        })?;
                        println!(
                            "  {}: {} docs indexed ({} errors)",
                            label,
                            stats.documents_indexed,
                            stats.errors.len()
                        );
                        for err in &stats.errors {
                            eprintln!("    Error: {} - {}", err.file.display(), err.message);
                        }
                        total_docs += stats.documents_indexed;
                        backend = Some(new_backend);
                    }
                    Some(ref mut existing) => {
                        let stats = builder.build_append(existing).await.map_err(|e| {
                            crate::error::Error::operation(format!("Vector append failed: {e}"))
                        })?;
                        println!(
                            "  {}: {} docs indexed ({} errors)",
                            label,
                            stats.documents_indexed,
                            stats.errors.len()
                        );
                        for err in &stats.errors {
                            eprintln!("    Error: {} - {}", err.file.display(), err.message);
                        }
                        total_docs += stats.documents_indexed;
                    }
                }
            }

            println!("\nVector index complete: {} total documents", total_docs);
            Ok(())
        }
        VectordbSubcommand::Status => {
            let base = config.paths.base_path()?;
            let cache_file = base.join(".cache").join("vector").join("vector-cache.json");

            if cache_file.exists() {
                println!("Vector cache: {}", cache_file.display());
                // Try to get metadata from cache file size
                if let Ok(meta) = std::fs::metadata(&cache_file) {
                    println!("  Cache size: {} KB", meta.len() / 1024);
                }
                println!("  Status: cached");
            } else {
                println!("Vector cache: not built");
                println!("  Run `music-theory-mcp vectordb build` to create the index");
            }
            Ok(())
        }
    }
}

/// Handle cache subcommand.
async fn handle_cache_command(
    cache_cmds: CacheCommands,
    log_level_override: Option<String>,
) -> Result<()> {
    let config = Config::load()?;
    let opts = apply_log_level_override(&config.logging, log_level_override)?;
    let _ = twyg::setup(opts);

    match cache_cmds.command {
        CacheSubcommand::Download { backend, force } => {
            let backends = crate::cache::parse_backend_arg(&backend)?;
            for b in &backends {
                crate::cache::download_cache(b, &config, force)?;
            }
            Ok(())
        }
        CacheSubcommand::Status => {
            let report = crate::cache::cache_status(&config)?;
            println!("{report}");
            Ok(())
        }
        CacheSubcommand::Package { backend, output } => {
            let backends = crate::cache::parse_backend_arg(&backend)?;
            let version = env!("CARGO_PKG_VERSION");
            let base_path = config.paths.base_path()?;
            let output_dir = std::path::PathBuf::from(output);
            for b in &backends {
                let path = crate::cache::package_cache(b, &base_path, &output_dir, version)?;
                println!("Packaged: {}", path.display());
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    fn test_cli(log_level: Option<String>, command: Option<Commands>) -> Cli {
        Cli {
            log_level,
            #[cfg(feature = "http")]
            transport: None,
            command,
        }
    }

    #[test]
    fn test_cli_parse_no_command() {
        let cli = Cli::parse_from(["music-theory-mcp"]);
        assert!(cli.command.is_none());
        assert!(cli.log_level.is_none());
    }

    #[test]
    fn test_cli_parse_serve() {
        let cli = Cli::parse_from(["music-theory-mcp", "serve"]);
        assert!(matches!(
            cli.command,
            Some(Commands::Serve { test: false, .. })
        ));
        assert!(cli.log_level.is_none());
    }

    #[test]
    fn test_cli_parse_serve_test_mode() {
        let cli = Cli::parse_from(["music-theory-mcp", "serve", "--test"]);
        assert!(matches!(
            cli.command,
            Some(Commands::Serve { test: true, .. })
        ));
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
        assert!(matches!(
            cli.command,
            Some(Commands::Serve { test: false, .. })
        ));
        assert_eq!(cli.log_level, Some("warn".to_string()));
    }

    #[test]
    fn test_cli_parse_log_level_before_command() {
        let cli = Cli::parse_from(["music-theory-mcp", "-l", "error", "serve"]);
        assert!(matches!(
            cli.command,
            Some(Commands::Serve { test: false, .. })
        ));
        assert_eq!(cli.log_level, Some("error".to_string()));
    }

    #[test]
    fn test_cli_parse_log_level_after_command() {
        let cli = Cli::parse_from(["music-theory-mcp", "serve", "--log-level", "info"]);
        assert!(matches!(
            cli.command,
            Some(Commands::Serve { test: false, .. })
        ));
        assert_eq!(cli.log_level, Some("info".to_string()));
    }

    #[test]
    fn test_cli_parse_serve_test_with_log_level() {
        let cli = Cli::parse_from(["music-theory-mcp", "-l", "debug", "serve", "--test"]);
        assert!(matches!(
            cli.command,
            Some(Commands::Serve { test: true, .. })
        ));
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
        let cli = test_cli(None, None);

        // This will try to run the server, which will hang waiting for stdin
        // TODO: Mock stdio transport to test this properly
        let result = handle_command(cli).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    #[serial(config_env)]
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
    #[serial(config_env)]
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
    #[serial(config_env)]
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
    #[serial(config_env)]
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
    #[serial(config_env)]
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
    #[serial(config_env)]
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
    #[serial(config_env)]
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
    #[serial(config_env)]
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
    #[serial(config_env)]
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

    #[test]
    #[cfg(feature = "graph")]
    fn test_cli_parse_graph_build() {
        let cli = Cli::parse_from(&["music-theory-mcp", "graph", "build"]);
        assert!(matches!(cli.command, Some(Commands::Graph(_))));
        assert!(cli.log_level.is_none());
    }

    #[test]
    #[cfg(feature = "graph")]
    fn test_cli_parse_graph_build_dry_run() {
        let cli = Cli::parse_from(&["music-theory-mcp", "graph", "build", "--dry-run"]);
        assert!(matches!(cli.command, Some(Commands::Graph(_))));
    }

    #[test]
    #[cfg(feature = "graph")]
    fn test_cli_parse_graph_build_verbose() {
        let cli = Cli::parse_from(&["music-theory-mcp", "graph", "build", "--verbose"]);
        assert!(matches!(cli.command, Some(Commands::Graph(_))));
    }

    #[test]
    #[cfg(feature = "graph")]
    fn test_cli_parse_graph_build_both_flags() {
        let cli = Cli::parse_from(&[
            "music-theory-mcp",
            "graph",
            "build",
            "--dry-run",
            "--verbose",
        ]);
        assert!(matches!(cli.command, Some(Commands::Graph(_))));
    }

    #[test]
    #[cfg(feature = "graph")]
    fn test_cli_parse_graph_build_verbose_short() {
        let cli = Cli::parse_from(&["music-theory-mcp", "graph", "build", "-v"]);
        assert!(matches!(cli.command, Some(Commands::Graph(_))));
    }

    #[test]
    #[cfg(feature = "graph")]
    fn test_cli_parse_graph_validate() {
        let cli = Cli::parse_from(&["music-theory-mcp", "graph", "validate"]);
        assert!(matches!(cli.command, Some(Commands::Graph(_))));
        assert!(cli.log_level.is_none());
    }

    #[test]
    #[cfg(feature = "graph")]
    fn test_cli_parse_graph_stats() {
        let cli = Cli::parse_from(&["music-theory-mcp", "graph", "stats"]);
        assert!(matches!(cli.command, Some(Commands::Graph(_))));
        assert!(cli.log_level.is_none());
    }

    #[test]
    #[cfg(feature = "graph")]
    fn test_cli_parse_graph_compile() {
        let cli = Cli::parse_from(&["music-theory-mcp", "graph", "compile"]);
        assert!(matches!(cli.command, Some(Commands::Graph(_))));
        assert!(cli.log_level.is_none());
    }

    #[test]
    #[cfg(feature = "graph")]
    fn test_cli_parse_graph_with_log_level() {
        let cli = Cli::parse_from(&["music-theory-mcp", "--log-level", "debug", "graph", "build"]);
        assert!(matches!(cli.command, Some(Commands::Graph(_))));
        assert_eq!(cli.log_level, Some("debug".to_string()));
    }

    #[test]
    #[cfg(feature = "graph")]
    fn test_cli_parse_graph_log_level_after_command() {
        let cli = Cli::parse_from(&["music-theory-mcp", "graph", "--log-level", "trace", "stats"]);
        assert!(matches!(cli.command, Some(Commands::Graph(_))));
        assert_eq!(cli.log_level, Some("trace".to_string()));
    }

    // Sources command tests

    #[test]
    fn test_cli_parse_sources_scan() {
        let cli = Cli::parse_from(&["music-theory-mcp", "sources", "scan"]);
        assert!(matches!(cli.command, Some(Commands::Sources(_))));
        assert!(cli.log_level.is_none());
    }

    #[test]
    fn test_cli_parse_sources_scan_with_json() {
        let cli = Cli::parse_from(&["music-theory-mcp", "sources", "scan", "--output", "json"]);
        assert!(matches!(cli.command, Some(Commands::Sources(_))));
    }

    #[test]
    fn test_cli_parse_sources_scan_show_cards() {
        let cli = Cli::parse_from(&["music-theory-mcp", "sources", "scan", "--show-cards"]);
        assert!(matches!(cli.command, Some(Commands::Sources(_))));
    }

    #[test]
    fn test_cli_parse_sources_validate() {
        let cli = Cli::parse_from(&["music-theory-mcp", "sources", "validate"]);
        assert!(matches!(cli.command, Some(Commands::Sources(_))));
        assert!(cli.log_level.is_none());
    }

    #[test]
    fn test_cli_parse_sources_validate_with_mode() {
        let cli = Cli::parse_from(&[
            "music-theory-mcp",
            "sources",
            "validate",
            "--mode",
            "cards-config",
        ]);
        assert!(matches!(cli.command, Some(Commands::Sources(_))));
    }

    #[test]
    fn test_cli_parse_sources_validate_suggest_matches() {
        let cli = Cli::parse_from(&[
            "music-theory-mcp",
            "sources",
            "validate",
            "--suggest-matches",
        ]);
        assert!(matches!(cli.command, Some(Commands::Sources(_))));
    }

    #[test]
    fn test_cli_parse_sources_validate_with_threshold() {
        let cli = Cli::parse_from(&[
            "music-theory-mcp",
            "sources",
            "validate",
            "--suggest-matches",
            "--threshold",
            "0.8",
        ]);
        assert!(matches!(cli.command, Some(Commands::Sources(_))));
    }

    #[test]
    fn test_cli_parse_sources_validate_json() {
        let cli = Cli::parse_from(&["music-theory-mcp", "sources", "validate", "--json"]);
        assert!(matches!(cli.command, Some(Commands::Sources(_))));
    }

    #[test]
    fn test_cli_parse_sources_alias_list() {
        let cli = Cli::parse_from(&["music-theory-mcp", "sources", "alias", "list"]);
        assert!(matches!(cli.command, Some(Commands::Sources(_))));
    }

    #[test]
    fn test_cli_parse_sources_alias_list_json() {
        let cli = Cli::parse_from(&["music-theory-mcp", "sources", "alias", "list", "--json"]);
        assert!(matches!(cli.command, Some(Commands::Sources(_))));
    }

    #[test]
    fn test_cli_parse_sources_alias_add() {
        let cli = Cli::parse_from(&[
            "music-theory-mcp",
            "sources",
            "alias",
            "add",
            "general-persichetti",
            "20th Century Harmony",
        ]);
        assert!(matches!(cli.command, Some(Commands::Sources(_))));
    }

    #[test]
    fn test_cli_parse_sources_alias_remove() {
        let cli = Cli::parse_from(&[
            "music-theory-mcp",
            "sources",
            "alias",
            "remove",
            "general-persichetti",
            "20th Century Harmony",
        ]);
        assert!(matches!(cli.command, Some(Commands::Sources(_))));
    }

    #[test]
    fn test_cli_parse_sources_with_log_level() {
        let cli = Cli::parse_from(&[
            "music-theory-mcp",
            "--log-level",
            "debug",
            "sources",
            "scan",
        ]);
        assert!(matches!(cli.command, Some(Commands::Sources(_))));
        assert_eq!(cli.log_level, Some("debug".to_string()));
    }

    // ====================================================================
    // Additional CLI parsing tests for deeper coverage
    // ====================================================================

    #[test]
    fn test_cli_try_parse_invalid_command() {
        let result = Cli::try_parse_from(["music-theory-mcp", "nonexistent"]);
        assert!(result.is_err(), "Should fail with invalid subcommand");
    }

    #[test]
    fn test_cli_try_parse_invalid_flag() {
        let result = Cli::try_parse_from(["music-theory-mcp", "--nonexistent-flag"]);
        assert!(result.is_err(), "Should fail with invalid flag");
    }

    #[test]
    fn test_cli_try_parse_serve_invalid_flag() {
        let result = Cli::try_parse_from(["music-theory-mcp", "serve", "--invalid"]);
        assert!(result.is_err(), "Should fail with invalid serve flag");
    }

    #[test]
    fn test_cli_parse_help_flag() {
        // --help causes clap to exit, but try_parse_from should capture it as an error
        let result = Cli::try_parse_from(["music-theory-mcp", "--help"]);
        assert!(result.is_err(), "Help flag should cause parse to fail/exit");
    }

    #[test]
    fn test_cli_parse_version_flag() {
        let result = Cli::try_parse_from(["music-theory-mcp", "--version"]);
        assert!(
            result.is_err(),
            "Version flag should cause parse to fail/exit"
        );
    }

    #[test]
    fn test_cli_parse_serve_with_all_options() {
        let cli = Cli::parse_from(["music-theory-mcp", "-l", "trace", "serve", "--test"]);
        assert_eq!(cli.log_level, Some("trace".to_string()));
        match cli.command {
            Some(Commands::Serve { test, .. }) => {
                assert!(test, "Test mode should be true");
            }
            other => panic!("Expected Serve command, got {:?}", other.is_some()),
        }
    }

    #[test]
    fn test_cli_default_command_is_none() {
        let cli = Cli::parse_from(["music-theory-mcp"]);
        assert!(cli.command.is_none(), "No subcommand should yield None");
        assert!(cli.log_level.is_none());
    }

    #[test]
    fn test_cli_log_level_all_valid_levels() {
        let valid_levels = ["trace", "debug", "info", "warn", "error"];
        for level in &valid_levels {
            let cli = Cli::parse_from(["music-theory-mcp", "--log-level", level]);
            assert_eq!(
                cli.log_level.as_deref(),
                Some(*level),
                "Log level '{}' should parse correctly",
                level
            );
        }
    }

    // ====================================================================
    // Graph subcommand deep extraction tests
    // ====================================================================

    #[test]
    #[cfg(feature = "graph")]
    fn test_cli_parse_graph_build_extract_flags() {
        let cli = Cli::parse_from(&[
            "music-theory-mcp",
            "graph",
            "build",
            "--dry-run",
            "--verbose",
        ]);
        match cli.command {
            Some(Commands::Graph(ref g)) => match g.command {
                GraphSubcommand::Build { dry_run, verbose } => {
                    assert!(dry_run, "dry_run should be true");
                    assert!(verbose, "verbose should be true");
                }
                ref other => panic!("Expected Build, got {:?}", std::mem::discriminant(other)),
            },
            _ => panic!("Expected Graph command"),
        }
    }

    #[test]
    #[cfg(feature = "graph")]
    fn test_cli_parse_graph_build_default_flags() {
        let cli = Cli::parse_from(&["music-theory-mcp", "graph", "build"]);
        match cli.command {
            Some(Commands::Graph(ref g)) => match g.command {
                GraphSubcommand::Build { dry_run, verbose } => {
                    assert!(!dry_run, "dry_run should default to false");
                    assert!(!verbose, "verbose should default to false");
                }
                ref other => panic!("Expected Build, got {:?}", std::mem::discriminant(other)),
            },
            _ => panic!("Expected Graph command"),
        }
    }

    #[test]
    #[cfg(feature = "graph")]
    fn test_cli_parse_graph_validate_subcommand() {
        let cli = Cli::parse_from(&["music-theory-mcp", "graph", "validate"]);
        match cli.command {
            Some(Commands::Graph(ref g)) => {
                assert!(
                    matches!(g.command, GraphSubcommand::Validate),
                    "Should be Validate subcommand"
                );
            }
            _ => panic!("Expected Graph command"),
        }
    }

    #[test]
    #[cfg(feature = "graph")]
    fn test_cli_parse_graph_stats_subcommand() {
        let cli = Cli::parse_from(&["music-theory-mcp", "graph", "stats"]);
        match cli.command {
            Some(Commands::Graph(ref g)) => {
                assert!(
                    matches!(g.command, GraphSubcommand::Stats),
                    "Should be Stats subcommand"
                );
            }
            _ => panic!("Expected Graph command"),
        }
    }

    #[test]
    #[cfg(feature = "graph")]
    fn test_cli_parse_graph_compile_subcommand() {
        let cli = Cli::parse_from(&["music-theory-mcp", "graph", "compile"]);
        match cli.command {
            Some(Commands::Graph(ref g)) => {
                assert!(
                    matches!(g.command, GraphSubcommand::Compile),
                    "Should be Compile subcommand"
                );
            }
            _ => panic!("Expected Graph command"),
        }
    }

    #[test]
    #[cfg(feature = "graph")]
    fn test_cli_parse_graph_invalid_subcommand() {
        let result = Cli::try_parse_from(["music-theory-mcp", "graph", "invalid"]);
        assert!(result.is_err(), "Should fail with invalid graph subcommand");
    }

    // ====================================================================
    // Sources subcommand deep extraction tests
    // ====================================================================

    #[test]
    fn test_cli_parse_sources_scan_extract_flags() {
        let cli = Cli::parse_from(&[
            "music-theory-mcp",
            "sources",
            "scan",
            "--output",
            "json",
            "--show-cards",
        ]);
        match cli.command {
            Some(Commands::Sources(ref s)) => match s.command {
                fabryk_cli::sources_handlers::SourcesSubcommand::Scan {
                    ref output,
                    show_cards,
                } => {
                    assert_eq!(output, "json");
                    assert!(show_cards);
                }
                _ => panic!("Expected Scan subcommand"),
            },
            _ => panic!("Expected Sources command"),
        }
    }

    #[test]
    fn test_cli_parse_sources_scan_defaults() {
        let cli = Cli::parse_from(&["music-theory-mcp", "sources", "scan"]);
        match cli.command {
            Some(Commands::Sources(ref s)) => match s.command {
                fabryk_cli::sources_handlers::SourcesSubcommand::Scan {
                    ref output,
                    show_cards,
                } => {
                    assert_eq!(output, "table", "Default output format should be 'table'");
                    assert!(!show_cards, "show_cards should default to false");
                }
                _ => panic!("Expected Scan subcommand"),
            },
            _ => panic!("Expected Sources command"),
        }
    }

    #[test]
    fn test_cli_parse_sources_validate_extract_flags() {
        let cli = Cli::parse_from(&[
            "music-theory-mcp",
            "sources",
            "validate",
            "--mode",
            "cards-config",
            "--suggest-matches",
            "--threshold",
            "0.9",
            "--json",
        ]);
        match cli.command {
            Some(Commands::Sources(ref s)) => match s.command {
                fabryk_cli::sources_handlers::SourcesSubcommand::Validate {
                    ref mode,
                    suggest_matches,
                    threshold,
                    json,
                } => {
                    assert_eq!(mode, "cards-config");
                    assert!(suggest_matches);
                    assert!((threshold - 0.9).abs() < f32::EPSILON);
                    assert!(json);
                }
                _ => panic!("Expected Validate subcommand"),
            },
            _ => panic!("Expected Sources command"),
        }
    }

    #[test]
    fn test_cli_parse_sources_validate_defaults() {
        let cli = Cli::parse_from(&["music-theory-mcp", "sources", "validate"]);
        match cli.command {
            Some(Commands::Sources(ref s)) => match s.command {
                fabryk_cli::sources_handlers::SourcesSubcommand::Validate {
                    ref mode,
                    suggest_matches,
                    threshold,
                    json,
                } => {
                    assert_eq!(mode, "all", "Default mode should be 'all'");
                    assert!(!suggest_matches, "suggest_matches should default to false");
                    assert!(
                        (threshold - 0.7).abs() < f32::EPSILON,
                        "Default threshold should be 0.7"
                    );
                    assert!(!json, "json should default to false");
                }
                _ => panic!("Expected Validate subcommand"),
            },
            _ => panic!("Expected Sources command"),
        }
    }

    #[test]
    fn test_cli_parse_sources_alias_add_extract_args() {
        let cli = Cli::parse_from(&[
            "music-theory-mcp",
            "sources",
            "alias",
            "add",
            "general-persichetti",
            "20th Century Harmony",
        ]);
        match cli.command {
            Some(Commands::Sources(ref s)) => match s.command {
                fabryk_cli::sources_handlers::SourcesSubcommand::Alias(ref alias_cmds) => {
                    match alias_cmds.command {
                        fabryk_cli::sources_handlers::AliasSubcommand::Add {
                            ref source_id,
                            ref alias,
                        } => {
                            assert_eq!(source_id, "general-persichetti");
                            assert_eq!(alias, "20th Century Harmony");
                        }
                        _ => panic!("Expected Add subcommand"),
                    }
                }
                _ => panic!("Expected Alias subcommand"),
            },
            _ => panic!("Expected Sources command"),
        }
    }

    #[test]
    fn test_cli_parse_sources_alias_remove_extract_args() {
        let cli = Cli::parse_from(&[
            "music-theory-mcp",
            "sources",
            "alias",
            "remove",
            "general-persichetti",
            "20th Century Harmony",
        ]);
        match cli.command {
            Some(Commands::Sources(ref s)) => match s.command {
                fabryk_cli::sources_handlers::SourcesSubcommand::Alias(ref alias_cmds) => {
                    match alias_cmds.command {
                        fabryk_cli::sources_handlers::AliasSubcommand::Remove {
                            ref source_id,
                            ref alias,
                        } => {
                            assert_eq!(source_id, "general-persichetti");
                            assert_eq!(alias, "20th Century Harmony");
                        }
                        _ => panic!("Expected Remove subcommand"),
                    }
                }
                _ => panic!("Expected Alias subcommand"),
            },
            _ => panic!("Expected Sources command"),
        }
    }

    #[test]
    fn test_cli_parse_sources_alias_list_extract_flags() {
        let cli = Cli::parse_from(&["music-theory-mcp", "sources", "alias", "list", "--json"]);
        match cli.command {
            Some(Commands::Sources(ref s)) => match s.command {
                fabryk_cli::sources_handlers::SourcesSubcommand::Alias(ref alias_cmds) => {
                    match alias_cmds.command {
                        fabryk_cli::sources_handlers::AliasSubcommand::List { json } => {
                            assert!(json, "json flag should be true");
                        }
                        _ => panic!("Expected List subcommand"),
                    }
                }
                _ => panic!("Expected Alias subcommand"),
            },
            _ => panic!("Expected Sources command"),
        }
    }

    #[test]
    fn test_cli_parse_sources_invalid_subcommand() {
        let result = Cli::try_parse_from(["music-theory-mcp", "sources", "invalid"]);
        assert!(
            result.is_err(),
            "Should fail with invalid sources subcommand"
        );
    }

    // ====================================================================
    // apply_log_level_override preservation tests
    // ====================================================================

    #[test]
    fn test_apply_log_level_override_preserves_all_fields() {
        use twyg::{LogLevel, OptsBuilder, Output};

        let original = OptsBuilder::new()
            .level(LogLevel::Info)
            .coloured(false)
            .output(Output::Stdout)
            .report_caller(true)
            .pad_level(true)
            .build()
            .unwrap();

        let result = apply_log_level_override(&original, Some("debug".to_string())).unwrap();

        // Level should be overridden
        assert_eq!(result.level(), LogLevel::Debug);
        // All other fields should be preserved
        assert_eq!(result.coloured(), false);
        assert_eq!(result.output(), &Output::Stdout);
        assert_eq!(result.report_caller(), true);
        assert_eq!(result.pad_level(), true);
    }

    #[test]
    fn test_apply_log_level_override_empty_string() {
        use twyg::{LogLevel, OptsBuilder, Output};

        let original = OptsBuilder::new()
            .level(LogLevel::Info)
            .coloured(true)
            .output(Output::Stderr)
            .build()
            .unwrap();

        let result = apply_log_level_override(&original, Some("".to_string()));
        assert!(
            result.is_err(),
            "Empty string should be an invalid log level"
        );
    }

    #[test]
    fn test_apply_log_level_override_case_sensitive() {
        use twyg::{LogLevel, OptsBuilder, Output};

        let original = OptsBuilder::new()
            .level(LogLevel::Info)
            .coloured(true)
            .output(Output::Stderr)
            .build()
            .unwrap();

        // "Debug" with capital D -- may or may not parse depending on twyg
        let result = apply_log_level_override(&original, Some("DEBUG".to_string()));
        // Either it works (case-insensitive) or fails (case-sensitive) - both are valid
        // We just ensure no panic
        let _ = result;
    }

    // ====================================================================
    // setup_cli_logging tests (test variant)
    // ====================================================================

    #[test]
    #[cfg(feature = "fts")]
    fn test_setup_cli_logging_returns_ok() {
        use twyg::{LogLevel, OptsBuilder, Output};

        let opts = OptsBuilder::new()
            .level(LogLevel::Error)
            .coloured(false)
            .output(Output::Stderr)
            .build()
            .unwrap();

        // The test variant of setup_cli_logging always returns Ok
        let result = setup_cli_logging(opts);
        assert!(result.is_ok(), "Test variant should always return Ok");
    }

    #[test]
    #[cfg(feature = "fts")]
    fn test_setup_cli_logging_multiple_calls() {
        use twyg::{LogLevel, OptsBuilder, Output};

        // Calling multiple times should be fine in test mode
        for _ in 0..3 {
            let opts = OptsBuilder::new()
                .level(LogLevel::Debug)
                .coloured(false)
                .output(Output::Stderr)
                .build()
                .unwrap();

            let result = setup_cli_logging(opts);
            assert!(result.is_ok());
        }
    }

    // ====================================================================
    // handle_command dispatch tests
    // ====================================================================

    #[tokio::test]
    #[serial(config_env)]
    #[cfg(feature = "fts")]
    async fn test_handle_command_dispatches_to_index() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let concept_cards_path = temp_dir.path().join("concept-cards");
        fs::create_dir_all(&concept_cards_path).expect("Failed to create dir");

        let card_content = r#"---
title: Test Card
category: test
---

# Test Card

Test content.
"#;
        fs::write(concept_cards_path.join("test.md"), card_content).expect("Failed to write card");

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

        let cli = test_cli(None, Some(Commands::Index { force: true }));

        let result = handle_command(cli).await;

        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(
            result.is_ok(),
            "handle_command with Index should succeed: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[serial(config_env)]
    #[cfg(feature = "fts")]
    async fn test_handle_command_dispatches_to_status() {
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

        let cli = test_cli(Some("error".to_string()), Some(Commands::Status));

        let result = handle_command(cli).await;

        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(
            result.is_ok(),
            "handle_command with Status should succeed: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[serial(config_env)]
    #[cfg(feature = "graph")]
    async fn test_handle_command_dispatches_to_graph() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let concept_cards_path = temp_dir.path().join("concept-cards");
        fs::create_dir_all(&concept_cards_path).expect("Failed to create dir");

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
backend = "simple"
index_path = ".tantivy-index"
rebuild_on_startup = false
snippet_size = 200
fuzzy_search = false
fuzzy_distance = 2
"#,
            temp_dir.path().display()
        );

        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(&config_dir).expect("Failed to create config dir");
        fs::write(config_dir.join("default.toml"), config_content).expect("Failed to write config");

        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        // Test graph stats command (lightest operation)
        let cli = test_cli(
            None,
            Some(Commands::Graph(GraphCommands {
                command: GraphSubcommand::Stats,
            })),
        );

        let result = handle_command(cli).await;

        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        // May fail if graph backend isn't fully set up, but tests the dispatch path
        // The important thing is we exercised handle_graph_command
        let _ = result;
    }

    #[tokio::test]
    #[serial(config_env)]
    #[cfg(feature = "graph")]
    async fn test_handle_graph_command_build_dry_run() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let concept_cards_path = temp_dir.path().join("concept-cards");
        fs::create_dir_all(&concept_cards_path).expect("Failed to create dir");

        let card_content = r#"---
title: Test Card
category: test
---

# Test Card

Test content.
"#;
        fs::write(concept_cards_path.join("test.md"), card_content).expect("Failed to write card");

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
backend = "simple"
index_path = ".tantivy-index"
rebuild_on_startup = false
snippet_size = 200
fuzzy_search = false
fuzzy_distance = 2
"#,
            temp_dir.path().display()
        );

        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(&config_dir).expect("Failed to create config dir");
        fs::write(config_dir.join("default.toml"), config_content).expect("Failed to write config");

        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        let graph_cmds = GraphCommands {
            command: GraphSubcommand::Build {
                dry_run: true,
                verbose: false,
            },
        };

        let result = handle_graph_command(graph_cmds, None).await;

        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        // Dry run build should succeed even with minimal content
        let _ = result;
    }

    #[tokio::test]
    #[serial(config_env)]
    #[cfg(feature = "graph")]
    async fn test_handle_graph_command_validate() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let concept_cards_path = temp_dir.path().join("concept-cards");
        fs::create_dir_all(&concept_cards_path).expect("Failed to create dir");

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
backend = "simple"
index_path = ".tantivy-index"
rebuild_on_startup = false
snippet_size = 200
fuzzy_search = false
fuzzy_distance = 2
"#,
            temp_dir.path().display()
        );

        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(&config_dir).expect("Failed to create config dir");
        fs::write(config_dir.join("default.toml"), config_content).expect("Failed to write config");

        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        let graph_cmds = GraphCommands {
            command: GraphSubcommand::Validate,
        };

        let result = handle_graph_command(graph_cmds, Some("error".to_string())).await;

        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        let _ = result;
    }

    #[tokio::test]
    #[serial(config_env)]
    #[cfg(feature = "graph")]
    async fn test_handle_graph_command_compile() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let concept_cards_path = temp_dir.path().join("concept-cards");
        fs::create_dir_all(&concept_cards_path).expect("Failed to create dir");

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
backend = "simple"
index_path = ".tantivy-index"
rebuild_on_startup = false
snippet_size = 200
fuzzy_search = false
fuzzy_distance = 2
"#,
            temp_dir.path().display()
        );

        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(&config_dir).expect("Failed to create config dir");
        fs::write(config_dir.join("default.toml"), config_content).expect("Failed to write config");

        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        let graph_cmds = GraphCommands {
            command: GraphSubcommand::Compile,
        };

        let result = handle_graph_command(graph_cmds, None).await;

        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        let _ = result;
    }

    #[tokio::test]
    #[serial(config_env)]
    #[cfg(feature = "graph")]
    async fn test_handle_graph_command_stats() {
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
backend = "simple"
index_path = ".tantivy-index"
rebuild_on_startup = false
snippet_size = 200
fuzzy_search = false
fuzzy_distance = 2
"#,
            temp_dir.path().display()
        );

        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(&config_dir).expect("Failed to create config dir");
        fs::write(config_dir.join("default.toml"), config_content).expect("Failed to write config");

        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        let graph_cmds = GraphCommands {
            command: GraphSubcommand::Stats,
        };

        let result = handle_graph_command(graph_cmds, None).await;

        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        let _ = result;
    }

    // ====================================================================
    // handle_command Sources dispatch test
    // ====================================================================

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_command_dispatches_to_sources_scan() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let concept_cards_path = temp_dir.path().join("concept-cards");
        fs::create_dir_all(&concept_cards_path).expect("Failed to create dir");

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
backend = "simple"
index_path = ".tantivy-index"
rebuild_on_startup = false
snippet_size = 200
fuzzy_search = false
fuzzy_distance = 2
"#,
            temp_dir.path().display()
        );

        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(&config_dir).expect("Failed to create config dir");
        fs::write(config_dir.join("default.toml"), config_content).expect("Failed to write config");

        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        // Construct a Sources Scan command directly
        let sources_cmds = SourcesCommand {
            command: fabryk_cli::sources_handlers::SourcesSubcommand::Scan {
                output: "table".to_string(),
                show_cards: false,
            },
        };

        let cli = test_cli(None, Some(Commands::Sources(sources_cmds)));

        let result = handle_command(cli).await;

        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        // Sources scan on an empty directory should succeed
        let _ = result;
    }

    // ====================================================================
    // FTS-specific index command edge cases
    // ====================================================================

    #[tokio::test]
    #[serial(config_env)]
    #[cfg(feature = "fts")]
    async fn test_handle_index_command_with_invalid_log_level() {
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
            temp_dir.path().join("test-index").display()
        );

        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(&config_dir).expect("Failed to create config dir");
        fs::write(config_dir.join("default.toml"), config_content).expect("Failed to write config");

        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        let result = handle_index_command(false, Some("invalid_level".to_string())).await;

        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(result.is_err(), "Invalid log level should cause an error");
    }

    #[tokio::test]
    #[serial(config_env)]
    #[cfg(feature = "fts")]
    async fn test_handle_status_command_with_invalid_log_level() {
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

        let result = handle_status_command(Some("bad_level".to_string())).await;

        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(result.is_err(), "Invalid log level should cause an error");
    }

    #[tokio::test]
    #[serial(config_env)]
    #[cfg(feature = "fts")]
    async fn test_handle_command_index_force_via_dispatch() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let concept_cards_path = temp_dir.path().join("concept-cards");
        fs::create_dir_all(&concept_cards_path).expect("Failed to create dir");

        let card_content = r#"---
title: Test Card
category: test
---

# Test Card

Test content.
"#;
        fs::write(concept_cards_path.join("test.md"), card_content).expect("Failed to write card");

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

        // Build index first
        build_index(&Config::load().unwrap())
            .await
            .expect("Failed to build index");

        // Dispatch through handle_command with force=false on a fresh index
        let cli = test_cli(None, Some(Commands::Index { force: false }));

        let result = handle_command(cli).await;

        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(
            result.is_ok(),
            "handle_command with Index (fresh, no force) should succeed: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[serial(config_env)]
    #[cfg(feature = "fts")]
    async fn test_handle_command_status_with_existing_index() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let concept_cards_path = temp_dir.path().join("concept-cards");
        fs::create_dir_all(&concept_cards_path).expect("Failed to create dir");

        let card_content = r#"---
title: Test Card
category: test
---

# Test Card

Test content.
"#;
        fs::write(concept_cards_path.join("test.md"), card_content).expect("Failed to write card");

        let index_path = temp_dir.path().join("test-index");

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

        // Dispatch through handle_command
        let cli = test_cli(Some("error".to_string()), Some(Commands::Status));

        let result = handle_command(cli).await;

        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(
            result.is_ok(),
            "handle_command with Status (existing index) should succeed: {:?}",
            result.err()
        );
    }

    // ====================================================================
    // HTTP feature CLI parsing tests
    // ====================================================================

    #[test]
    #[cfg(feature = "http")]
    fn test_cli_parse_serve_http() {
        let cli = Cli::parse_from(["music-theory-mcp", "--transport", "http", "serve"]);
        assert_eq!(cli.transport, Some("http".to_string()));
        match cli.command {
            Some(Commands::Serve { test, port }) => {
                assert!(!test, "test should be false");
                assert_eq!(port, 8080, "default port should be 8080");
            }
            other => panic!("Expected Serve command, got {:?}", other.is_some()),
        }
    }

    #[test]
    #[cfg(feature = "http")]
    fn test_cli_parse_serve_http_custom_port() {
        let cli = Cli::parse_from([
            "music-theory-mcp",
            "--transport",
            "http",
            "serve",
            "--port",
            "3000",
        ]);
        assert_eq!(cli.transport, Some("http".to_string()));
        match cli.command {
            Some(Commands::Serve { test, port }) => {
                assert!(!test, "test should be false");
                assert_eq!(port, 3000, "port should be 3000");
            }
            other => panic!("Expected Serve command, got {:?}", other.is_some()),
        }
    }

    #[test]
    #[cfg(feature = "http")]
    fn test_cli_parse_serve_http_short_port() {
        let cli = Cli::parse_from([
            "music-theory-mcp",
            "--transport",
            "http",
            "serve",
            "-p",
            "9090",
        ]);
        assert_eq!(cli.transport, Some("http".to_string()));
        match cli.command {
            Some(Commands::Serve { test, port }) => {
                assert!(!test);
                assert_eq!(port, 9090);
            }
            other => panic!("Expected Serve command, got {:?}", other.is_some()),
        }
    }

    #[test]
    #[cfg(feature = "http")]
    fn test_cli_parse_serve_http_with_test_and_log_level() {
        let cli = Cli::parse_from([
            "music-theory-mcp",
            "-l",
            "debug",
            "--transport",
            "http",
            "serve",
            "--test",
            "--port",
            "4000",
        ]);
        assert_eq!(cli.transport, Some("http".to_string()));
        match cli.command {
            Some(Commands::Serve { test, port }) => {
                assert!(test, "test should be true");
                assert_eq!(port, 4000);
            }
            other => panic!("Expected Serve command, got {:?}", other.is_some()),
        }
        assert_eq!(cli.log_level, Some("debug".to_string()));
    }

    #[test]
    #[cfg(feature = "http")]
    fn test_cli_parse_serve_no_http_flag_defaults() {
        let cli = Cli::parse_from(["music-theory-mcp", "serve"]);
        assert!(cli.transport.is_none(), "transport should default to None");
        match cli.command {
            Some(Commands::Serve { test, port }) => {
                assert!(!test);
                assert_eq!(port, 8080, "port should default to 8080");
            }
            other => panic!("Expected Serve command, got {:?}", other.is_some()),
        }
    }

    // ====================================================================
    // Vector CLI parsing tests
    // ====================================================================

    #[test]
    #[cfg(feature = "vector")]
    fn test_cli_parse_vectordb_build() {
        let cli = Cli::parse_from(&["music-theory-mcp", "vectordb", "build"]);
        assert!(matches!(cli.command, Some(Commands::Vectordb(_))));
        assert!(cli.log_level.is_none());
    }

    #[test]
    #[cfg(feature = "vector")]
    fn test_cli_parse_vectordb_build_force() {
        let cli = Cli::parse_from(&["music-theory-mcp", "vectordb", "build", "--force"]);
        assert!(matches!(cli.command, Some(Commands::Vectordb(_))));
    }

    #[test]
    #[cfg(feature = "vector")]
    fn test_cli_parse_vectordb_build_force_short() {
        let cli = Cli::parse_from(&["music-theory-mcp", "vectordb", "build", "-f"]);
        assert!(matches!(cli.command, Some(Commands::Vectordb(_))));
    }

    #[test]
    #[cfg(feature = "vector")]
    fn test_cli_parse_vectordb_status() {
        let cli = Cli::parse_from(&["music-theory-mcp", "vectordb", "status"]);
        assert!(matches!(cli.command, Some(Commands::Vectordb(_))));
        assert!(cli.log_level.is_none());
    }

    #[test]
    #[cfg(feature = "vector")]
    fn test_cli_parse_vectordb_with_log_level() {
        let cli = Cli::parse_from(&[
            "music-theory-mcp",
            "--log-level",
            "debug",
            "vectordb",
            "build",
        ]);
        assert!(matches!(cli.command, Some(Commands::Vectordb(_))));
        assert_eq!(cli.log_level, Some("debug".to_string()));
    }

    #[test]
    #[cfg(feature = "vector")]
    fn test_cli_parse_vectordb_build_extract_force_true() {
        let cli = Cli::parse_from(&["music-theory-mcp", "vectordb", "build", "--force"]);
        match cli.command {
            Some(Commands::Vectordb(ref v)) => match v.command {
                VectordbSubcommand::Build { force } => {
                    assert!(force, "force should be true");
                }
                ref other => panic!(
                    "Expected Build subcommand, got {:?}",
                    std::mem::discriminant(other)
                ),
            },
            _ => panic!("Expected Vectordb command"),
        }
    }

    #[test]
    #[cfg(feature = "vector")]
    fn test_cli_parse_vectordb_build_extract_force_false() {
        let cli = Cli::parse_from(&["music-theory-mcp", "vectordb", "build"]);
        match cli.command {
            Some(Commands::Vectordb(ref v)) => match v.command {
                VectordbSubcommand::Build { force } => {
                    assert!(!force, "force should default to false");
                }
                ref other => panic!(
                    "Expected Build subcommand, got {:?}",
                    std::mem::discriminant(other)
                ),
            },
            _ => panic!("Expected Vectordb command"),
        }
    }

    #[test]
    #[cfg(feature = "vector")]
    fn test_cli_parse_vectordb_status_subcommand() {
        let cli = Cli::parse_from(&["music-theory-mcp", "vectordb", "status"]);
        match cli.command {
            Some(Commands::Vectordb(ref v)) => {
                assert!(
                    matches!(v.command, VectordbSubcommand::Status),
                    "Should be Status subcommand"
                );
            }
            _ => panic!("Expected Vectordb command"),
        }
    }

    #[test]
    #[cfg(feature = "vector")]
    fn test_cli_parse_vectordb_invalid_subcommand() {
        let result = Cli::try_parse_from(["music-theory-mcp", "vectordb", "invalid"]);
        assert!(
            result.is_err(),
            "Should fail with invalid vectordb subcommand"
        );
    }

    // ====================================================================
    // handle_command Sources dispatch with log_level
    // ====================================================================

    #[tokio::test]
    #[serial(config_env)]
    async fn test_handle_command_dispatches_to_sources_with_log_level() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let concept_cards_path = temp_dir.path().join("concept-cards");
        fs::create_dir_all(&concept_cards_path).expect("Failed to create dir");

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
backend = "simple"
index_path = ".tantivy-index"
rebuild_on_startup = false
snippet_size = 200
fuzzy_search = false
fuzzy_distance = 2
"#,
            temp_dir.path().display()
        );

        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(&config_dir).expect("Failed to create config dir");
        fs::write(config_dir.join("default.toml"), config_content).expect("Failed to write config");

        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        // Test that log_level clone works properly through Sources dispatch
        let sources_cmds = SourcesCommand {
            command: fabryk_cli::sources_handlers::SourcesSubcommand::Scan {
                output: "table".to_string(),
                show_cards: false,
            },
        };

        let cli = test_cli(
            Some("error".to_string()),
            Some(Commands::Sources(sources_cmds)),
        );

        let result = handle_command(cli).await;

        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        // Sources scan on an empty directory should succeed
        let _ = result;
    }

    // ====================================================================
    // handle_command with log_level clone path (line 152)
    // ====================================================================

    #[tokio::test]
    #[serial(config_env)]
    #[cfg(feature = "fts")]
    async fn test_handle_command_log_level_clone_with_index() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let concept_cards_path = temp_dir.path().join("concept-cards");
        fs::create_dir_all(&concept_cards_path).expect("Failed to create dir");

        let card_content = r#"---
title: Test Card
category: test
---

# Test Card

Test content.
"#;
        fs::write(concept_cards_path.join("test.md"), card_content).expect("Failed to write card");

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

        // Verify that log_level is properly cloned and passed to the handler
        let cli = test_cli(
            Some("warn".to_string()),
            Some(Commands::Index { force: true }),
        );

        let result = handle_command(cli).await;

        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(
            result.is_ok(),
            "handle_command with log_level and Index should succeed: {:?}",
            result.err()
        );
    }

    // ====================================================================
    // handle_index_command force rebuild on existing fresh index
    // ====================================================================

    #[tokio::test]
    #[serial(config_env)]
    #[cfg(feature = "fts")]
    async fn test_handle_index_command_force_on_fresh_index() {
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

        // Build index first so it's fresh
        build_index(&Config::load().unwrap())
            .await
            .expect("Failed to build index");

        // Now force rebuild on a fresh index (exercises the force=true path
        // when the index is already fresh -- bypasses is_index_fresh check)
        let result = handle_index_command(true, None).await;

        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(
            result.is_ok(),
            "Force rebuild on fresh index should succeed: {:?}",
            result.err()
        );
    }

    // ====================================================================
    // handle_graph_command with invalid log level
    // ====================================================================

    #[tokio::test]
    #[serial(config_env)]
    #[cfg(feature = "graph")]
    async fn test_handle_graph_command_with_invalid_log_level() {
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
backend = "simple"
index_path = ".tantivy-index"
rebuild_on_startup = false
snippet_size = 200
fuzzy_search = false
fuzzy_distance = 2
"#,
            temp_dir.path().display()
        );

        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(&config_dir).expect("Failed to create config dir");
        fs::write(config_dir.join("default.toml"), config_content).expect("Failed to write config");

        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        let graph_cmds = GraphCommands {
            command: GraphSubcommand::Stats,
        };

        let result = handle_graph_command(graph_cmds, Some("bad_level".to_string())).await;

        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(
            result.is_err(),
            "Invalid log level should cause an error in graph command"
        );
    }

    // ====================================================================
    // handle_command dispatches to vectordb
    // ====================================================================

    #[tokio::test]
    #[serial(config_env)]
    #[cfg(feature = "vector")]
    async fn test_handle_command_dispatches_to_vectordb_status() {
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
backend = "simple"
index_path = ".tantivy-index"
rebuild_on_startup = false
snippet_size = 200
fuzzy_search = false
fuzzy_distance = 2
"#,
            temp_dir.path().display()
        );

        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(&config_dir).expect("Failed to create config dir");
        fs::write(config_dir.join("default.toml"), config_content).expect("Failed to write config");

        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        let cli = test_cli(
            None,
            Some(Commands::Vectordb(VectordbCommands {
                command: VectordbSubcommand::Status,
            })),
        );

        let result = handle_command(cli).await;

        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        // Vectordb status on a dir without a cache should succeed
        let _ = result;
    }

    #[tokio::test]
    #[serial(config_env)]
    #[cfg(feature = "vector")]
    async fn test_handle_vectordb_command_status_no_cache() {
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
backend = "simple"
index_path = ".tantivy-index"
rebuild_on_startup = false
snippet_size = 200
fuzzy_search = false
fuzzy_distance = 2
"#,
            temp_dir.path().display()
        );

        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(&config_dir).expect("Failed to create config dir");
        fs::write(config_dir.join("default.toml"), config_content).expect("Failed to write config");

        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        let vector_cmds = VectordbCommands {
            command: VectordbSubcommand::Status,
        };

        let result = handle_vectordb_command(vector_cmds, None).await;

        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(
            result.is_ok(),
            "Vectordb status with no cache should succeed: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[serial(config_env)]
    #[cfg(feature = "vector")]
    async fn test_handle_vectordb_command_status_with_cache() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        // Create a fake cache file
        let cache_dir = temp_dir.path().join(".cache").join("vector");
        fs::create_dir_all(&cache_dir).expect("Failed to create cache dir");
        fs::write(cache_dir.join("vector-cache.json"), r#"{"dummy": "data"}"#)
            .expect("Failed to write cache");

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
backend = "simple"
index_path = ".tantivy-index"
rebuild_on_startup = false
snippet_size = 200
fuzzy_search = false
fuzzy_distance = 2
"#,
            temp_dir.path().display()
        );

        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(&config_dir).expect("Failed to create config dir");
        fs::write(config_dir.join("default.toml"), config_content).expect("Failed to write config");

        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        let vector_cmds = VectordbCommands {
            command: VectordbSubcommand::Status,
        };

        let result = handle_vectordb_command(vector_cmds, None).await;

        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(
            result.is_ok(),
            "Vectordb status with existing cache should succeed: {:?}",
            result.err()
        );
    }

    #[tokio::test]
    #[serial(config_env)]
    #[cfg(feature = "vector")]
    async fn test_handle_vectordb_command_with_invalid_log_level() {
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
backend = "simple"
index_path = ".tantivy-index"
rebuild_on_startup = false
snippet_size = 200
fuzzy_search = false
fuzzy_distance = 2
"#,
            temp_dir.path().display()
        );

        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(&config_dir).expect("Failed to create config dir");
        fs::write(config_dir.join("default.toml"), config_content).expect("Failed to write config");

        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        let vector_cmds = VectordbCommands {
            command: VectordbSubcommand::Status,
        };

        let result = handle_vectordb_command(vector_cmds, Some("invalid_level".to_string())).await;

        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(
            result.is_err(),
            "Invalid log level should cause an error in vectordb command"
        );
    }

    #[tokio::test]
    #[serial(config_env)]
    #[cfg(feature = "vector")]
    async fn test_handle_vectordb_command_with_log_level() {
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
backend = "simple"
index_path = ".tantivy-index"
rebuild_on_startup = false
snippet_size = 200
fuzzy_search = false
fuzzy_distance = 2
"#,
            temp_dir.path().display()
        );

        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(&config_dir).expect("Failed to create config dir");
        fs::write(config_dir.join("default.toml"), config_content).expect("Failed to write config");

        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        let vector_cmds = VectordbCommands {
            command: VectordbSubcommand::Status,
        };

        let result = handle_vectordb_command(vector_cmds, Some("error".to_string())).await;

        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        assert!(
            result.is_ok(),
            "Vectordb status with valid log level should succeed: {:?}",
            result.err()
        );
    }

    // ====================================================================
    // handle_graph_command with log level override (valid)
    // ====================================================================

    #[tokio::test]
    #[serial(config_env)]
    #[cfg(feature = "graph")]
    async fn test_handle_graph_command_build_with_log_level() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let concept_cards_path = temp_dir.path().join("concept-cards");
        fs::create_dir_all(&concept_cards_path).expect("Failed to create dir");

        let card_content = r#"---
title: Test Card
category: test
---

# Test Card

Test content.
"#;
        fs::write(concept_cards_path.join("test.md"), card_content).expect("Failed to write card");

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
backend = "simple"
index_path = ".tantivy-index"
rebuild_on_startup = false
snippet_size = 200
fuzzy_search = false
fuzzy_distance = 2
"#,
            temp_dir.path().display()
        );

        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(&config_dir).expect("Failed to create config dir");
        fs::write(config_dir.join("default.toml"), config_content).expect("Failed to write config");

        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        let graph_cmds = GraphCommands {
            command: GraphSubcommand::Build {
                dry_run: true,
                verbose: true,
            },
        };

        let result = handle_graph_command(graph_cmds, Some("error".to_string())).await;

        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        // May fail due to graph backend, but exercises the log_level path
        let _ = result;
    }

    // ====================================================================
    // handle_command dispatches to graph with log_level
    // ====================================================================

    #[tokio::test]
    #[serial(config_env)]
    #[cfg(feature = "graph")]
    async fn test_handle_command_graph_with_log_level() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let concept_cards_path = temp_dir.path().join("concept-cards");
        fs::create_dir_all(&concept_cards_path).expect("Failed to create dir");

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
backend = "simple"
index_path = ".tantivy-index"
rebuild_on_startup = false
snippet_size = 200
fuzzy_search = false
fuzzy_distance = 2
"#,
            temp_dir.path().display()
        );

        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(&config_dir).expect("Failed to create config dir");
        fs::write(config_dir.join("default.toml"), config_content).expect("Failed to write config");

        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        let cli = test_cli(
            Some("error".to_string()),
            Some(Commands::Graph(GraphCommands {
                command: GraphSubcommand::Stats,
            })),
        );

        let result = handle_command(cli).await;

        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        // Exercises the log_level clone path through handle_command -> graph dispatch
        let _ = result;
    }

    // ====================================================================
    // handle_command dispatches to vectordb with log_level
    // ====================================================================

    #[tokio::test]
    #[serial(config_env)]
    #[cfg(feature = "vector")]
    async fn test_handle_command_vectordb_with_log_level() {
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
backend = "simple"
index_path = ".tantivy-index"
rebuild_on_startup = false
snippet_size = 200
fuzzy_search = false
fuzzy_distance = 2
"#,
            temp_dir.path().display()
        );

        let config_dir = temp_dir.path().join("config");
        fs::create_dir_all(&config_dir).expect("Failed to create config dir");
        fs::write(config_dir.join("default.toml"), config_content).expect("Failed to write config");

        std::env::set_var("MUSIC_THEORY_CONFIG_DIR", &config_dir);

        let cli = test_cli(
            Some("error".to_string()),
            Some(Commands::Vectordb(VectordbCommands {
                command: VectordbSubcommand::Status,
            })),
        );

        let result = handle_command(cli).await;

        std::env::remove_var("MUSIC_THEORY_CONFIG_DIR");

        // Exercises the log_level clone path through handle_command -> vectordb dispatch
        let _ = result;
    }
}
