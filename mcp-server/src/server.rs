use rmcp::handler::server::ServerHandler;
use rmcp::model::{Implementation, ServerCapabilities, ServerInfo};

use crate::config::Config;

/// Music Theory MCP Server implementation.
pub struct MusicTheoryServer {
    pub config: Config,
}

impl MusicTheoryServer {
    /// Create a new server instance.
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl ServerHandler for MusicTheoryServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: Default::default(),
            capabilities: ServerCapabilities::default(),
            server_info: Implementation {
                name: self.config.server.name.clone(),
                title: None,
                version: self.config.server.version.clone(),
                icons: None,
                website_url: None,
            },
            instructions: Some(
                "Music Theory AI Skill - Access comprehensive music theory materials \
                 including source texts, concept cards, and topic guides."
                    .to_string(),
            ),
        }
    }
}
