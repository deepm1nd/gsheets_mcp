use gsheets_mcp::mcp_server::McpServer;
use gsheets_mcp::stdio_server::StdioServer;
use tokio::runtime::Runtime;

#[tokio::main]
async fn main() {
    let server = match McpServer::new().await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("[FATAL] MCP server init failed: {}", e);
            std::process::exit(1);
        }
    };
    let stdio = StdioServer::new(server.api_client.clone());
    if let Err(e) = stdio.run().await {
        eprintln!("[FATAL] Stdio server error: {}", e);
        std::process::exit(1);
    }
}
