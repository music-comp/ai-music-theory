use clap::Parser;

use music_theory_mcp::cli::{self, Cli};
use music_theory_mcp::error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    cli::handle_command(cli).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use music_theory_mcp::config;

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
