#!/usr/bin/env bash
# Script to install git hooks for pulumi-gestalt
# This script creates a pre-commit hook that runs "mise run pre-commit"

set -euo pipefail

HOOKS_DIR=".git/hooks"
PRE_COMMIT_HOOK="$HOOKS_DIR/pre-commit"

# Create hooks directory if it doesn't exist
mkdir -p "$HOOKS_DIR"

# Create the pre-commit hook
cat > "$PRE_COMMIT_HOOK" << 'EOF'
#!/usr/bin/env bash
# Pre-commit hook for pulumi-gestalt
# Runs "just fmt" via mise task to ensure code is formatted before commit

set -euo pipefail

# Check if mise is available
if ! command -v mise &> /dev/null; then
    echo "Warning: mise is not installed. Please install mise to enable pre-commit hooks."
    echo "See: https://mise.jdx.dev/getting-started.html"
    exit 0
fi

# Run the pre-commit task
echo "Running pre-commit hook: just fmt"
mise run pre-commit
EOF

# Make the hook executable
chmod +x "$PRE_COMMIT_HOOK"

echo "Git hooks installed successfully!"
echo "Pre-commit hook will now run 'just fmt' before each commit."
