//! Integration tests for gsheets_mcp: HTTP, stdio, SEE, and Google API flows
use std::process::{Command, Stdio};
use std::io::{Write, Read};

#[test]
fn http_server_list_spreadsheets() {
    let mut child = Command::new("cargo")
        .args(["run", "--bin", "gsheets_mcp_http"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start HTTP server");
    std::thread::sleep(std::time::Duration::from_secs(2));
    let resp = reqwest::blocking::get("http://127.0.0.1:8080/spreadsheets").unwrap();
    let json: serde_json::Value = resp.json().unwrap();
    assert!(json["success"].as_bool().unwrap());
    child.kill().ok();
}

#[test]
fn stdio_server_create_and_list() {
    let mut child = Command::new("cargo")
        .args(["run", "--bin", "gsheets_mcp_stdio"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start stdio server");
    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = child.stdout.take().unwrap();
    stdin.write_all(b"{\"action\":\"create_spreadsheet\",\"title\":\"TestSheet\"}\n").unwrap();
    stdin.write_all(b"{\"action\":\"list_spreadsheets\"}\n").unwrap();
    let mut buf = [0u8; 4096];
    let n = stdout.read(&mut buf).unwrap();
    let output = String::from_utf8_lossy(&buf[..n]);
    assert!(output.contains("success"));
    child.kill().ok();
}

#[test]
fn see_server_list_spreadsheets() {
    let mut child = Command::new("cargo")
        .args(["run", "--bin", "gsheets_mcp_see"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start SEE server");
    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = child.stdout.take().unwrap();
    stdin.write_all(b"{\"event\":\"list_spreadsheets\"}\n").unwrap();
    let mut buf = [0u8; 4096];
    let n = stdout.read(&mut buf).unwrap();
    let output = String::from_utf8_lossy(&buf[..n]);
    assert!(output.contains("ok"));
    child.kill().ok();
}
