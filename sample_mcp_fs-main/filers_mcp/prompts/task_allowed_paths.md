# Task Prompt: Implement AllowedPaths Struct and Validation Logic

## Objective
Implement the `AllowedPaths` struct and associated path validation logic to enforce access restrictions to allowed directories, as described in the architecture and development plan.

## Requirements
- Only allow file operations within explicitly configured directories.
- Canonicalize all paths and deny symlinks or traversal outside allowed roots.
- Provide clear error messages and audit logs for denied access.
- Write unit tests for all validation logic.

## Implementation Steps
1. Design the `AllowedPaths` struct to store a list of allowed directory roots (as canonicalized `PathBuf`s).
2. Implement a constructor that validates and canonicalizes all configured directories, rejecting non-existent or non-directory paths.
3. Implement a method to validate a given path:
   - Canonicalize the input path.
   - Check that it is within one of the allowed roots.
   - Deny access if the path escapes allowed roots via symlinks or traversal.
   - Return a clear error if validation fails.
4. Integrate audit logging for all denied access attempts, including the attempted path and reason for denial.
5. Write comprehensive unit tests:
   - Test exact matches, subdirectory matches, and disallowed paths.
   - Test symlink and traversal edge cases.
   - Test error messages and audit log output.

## Acceptance Criteria
- All file operations are restricted to allowed directories.
- All validation logic is covered by unit tests.
- All denied accesses are logged with clear error messages.

## References
- filers_mcp_architecture_specification.md (Section 3, 4, 5)
- filers_mcp_development_plan.md (Phase 1, Task Breakdown)
