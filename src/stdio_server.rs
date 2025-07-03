use std::sync::Arc;
use crate::models::{McpHandler, ListSpreadsheetsHandler, ListSpreadsheetsRequest, CreateSpreadsheetHandler, CreateSpreadsheetRequest};
use crate::google_api_client::GoogleApiClient;

pub async fn stdio_server_main(client: Arc<GoogleApiClient>) {
    use std::io::{self, BufRead, Write};
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line == "list" {
            let handler = ListSpreadsheetsHandler { client: client.clone() };
            let result = handler.handle(ListSpreadsheetsRequest).await;
            writeln!(stdout, "{:?}", result).unwrap();
        } else if line.starts_with("create ") {
            let title = line[7..].to_string();
            let handler = CreateSpreadsheetHandler { client: client.clone() };
            let req = CreateSpreadsheetRequest { title };
            let result = handler.handle(req).await;
            writeln!(stdout, "{:?}", result).unwrap();
        } else {
            writeln!(stdout, "Unknown command").unwrap();
        }
        stdout.flush().unwrap();
    }
}
