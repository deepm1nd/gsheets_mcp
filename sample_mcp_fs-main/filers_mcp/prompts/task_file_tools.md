# Task Prompt: Develop read_file, write_file, and list_directory Tools

## Objective
Implement the `read_file`, `write_file`, and `list_directory` tools for the MCP server, following the security and validation requirements.

## Requirements
- Each tool must validate all paths using the AllowedPaths logic.
- `read_file` must support head/tail/partial reads.
- `write_file` must handle content safely and atomically.
- `list_directory` must return metadata for all entries.
- All errors and denied accesses must be logged.
- Write unit tests for each tool.

## References
- filers_mcp_architecture_specification.md (Section 3, 4, 5)
- filers_mcp_development_plan.md (Phase 2, Task Breakdown)
