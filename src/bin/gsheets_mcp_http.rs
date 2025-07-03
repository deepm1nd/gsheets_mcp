use gsheets_mcp::mcp_server::McpServer;
use gsheets_mcp::http_server::HttpServer;
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
    let http = HttpServer::new(server.api_client.clone());
    let addr = std::env::var("GSMCP_HTTP_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    if let Err(e) = http.run(&addr).await {
        eprintln!("[FATAL] HTTP server error: {}", e);
        std::process::exit(1);
    }
}
