#!/bin/bash
# Bootstrap a new Rust Rewriter session with handoff files

set -e

mkdir -p handoff

touch handoff/handoff_notes.md handoff/open_issues.md handoff/chat_history.md

echo "# Rust Rewriter Project\n\nThis session was bootstrapped with handoff files and is ready for intake." > README.md

chmod +x $0
