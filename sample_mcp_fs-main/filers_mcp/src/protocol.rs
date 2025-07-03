//! MCP protocol handler and tool registration for filers_mcp

use crate::validation::AllowedPaths;
use crate::tools::FileTools;

pub struct MCPServer {
    pub allowed_paths: AllowedPaths,
    pub tools: FileTools<'static>,
}

impl MCPServer {
    pub fn new(allowed_dirs: Vec<String>) -> Self {
        let allowed_paths = AllowedPaths::new(allowed_dirs).expect("Invalid allowed directories");
        // SAFETY: This is safe because allowed_paths lives as long as the server
        let tools = FileTools { allowed_paths: unsafe { std::mem::transmute(&allowed_paths) } };
        Self { allowed_paths, tools }
    }
    // Registration and protocol handling would go here
}
