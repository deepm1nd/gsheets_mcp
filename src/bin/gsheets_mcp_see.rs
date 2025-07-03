use gsheets_mcp::mcp_server::McpServer;
use gsheets_mcp::see_server::SeeServer;
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
    let see = SeeServer::new(server.api_client.clone());
    if let Err(e) = see.run().await {
        eprintln!("[FATAL] SEE server error: {}", e);
        std::process::exit(1);
    }
}
