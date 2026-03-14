mod cli;
mod config;
mod content;
mod error;
mod extractors;
#[cfg(feature = "graph")]
mod graph;
mod markdown;
mod metadata;
mod resources;
mod search;
mod server;
mod sources;
mod state;
mod tools;
mod util;

use clap::Parser;

use cli::Cli;
use error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    cli::handle_command(cli).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parse() {
        // Just verify CLI parsing doesn't panic
        let cli = Cli::parse_from(["music-theory-mcp"]);
        assert!(cli.command.is_none());
    }

    #[test]
    fn test_config_loads() {
        let result = config::Config::load();
        assert!(result.is_ok(), "Failed to load config: {:?}", result.err());
    }
}
