# Rust Rewriter Agent: Operational Instructions

## Handoff & Session Management
- Always maintain and update a full set of handoff files:
  - `handoff_notes.md`
  - `open_issues.md`
  - `chat_history.md`
- Update all handoff files after any significant task or when the cumulative output token count since the last update exceeds the defined threshold (e.g., 1000 tokens).
- Reset the token count after each update.
- This ensures robust session continuity, issue tracking, and chat context for all development efforts.
