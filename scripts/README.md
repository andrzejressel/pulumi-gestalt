# Scripts Directory

This directory contains utility scripts for the pulumi-gestalt project.

## install-git-hooks.sh

Installs git hooks for the repository, specifically a pre-commit hook that runs `just fmt` to ensure all code is properly formatted before committing.

### Usage

```bash
./scripts/install-git-hooks.sh
```

### Prerequisites

- `mise` must be installed (see main project README or AGENTS.md for installation instructions)
- The repository must be cloned with `.git` directory present

### What it does

The script creates a `.git/hooks/pre-commit` file that:
1. Checks if `mise` is installed
2. Runs the `pre-commit` mise task (defined in `.mise/tasks/pre-commit`)
3. Executes `just fmt` to format all code

If `mise` is not available, the hook will show a warning but won't block the commit, allowing developers to work without mise if needed.
