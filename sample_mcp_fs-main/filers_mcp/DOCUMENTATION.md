# filers_mcp User and Developer Documentation

## Overview
filers_mcp is a secure, auditable filesystem MCP server. It restricts all file operations to explicitly allowed directories, logs all denied accesses, and provides robust validation and error handling.

## Configuration
- Edit `example_config.toml` to set allowed directories and server options.
- Place your config at `/etc/filers_mcp/config.toml` for deployment.

## Running
- Build with `cargo build --release`.
- Deploy using `deploy.sh` or manually copy the binary and config.
- Start the server: `filers_mcp --config /etc/filers_mcp/config.toml`

## Security Model
- All file operations are validated against allowed directories.
- Symlinks and traversal outside allowed roots are denied and logged.
- All denied accesses and errors are logged for auditability.

## Extending
- Add new tools by implementing methods in `tools.rs` and registering them in `protocol.rs`.
- All new tools must use `AllowedPaths` for path validation.

## Testing
- Run `cargo test` to execute all unit and integration tests.

## References
- See `filers_mcp_architecture_specification.md` and `filers_mcp_development_plan.md` for design and implementation details.
