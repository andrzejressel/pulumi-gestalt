# Agent Instructions for the Pulumi Language Plugin

This directory contains the source code for the Go-based Pulumi language plugin for Gestalt.

## Development Workflow

The development workflow for this plugin is managed by the `justfile` in this directory.

### Common Commands

*   **Build and install the plugin locally**:
    ```bash
    just build
    ```
    This command runs `go build` and then uses `pulumi plugin install` to make it available for local testing.

*   **Format the Go code**:
    ```bash
    just fmt
    ```
    This runs `go fmt .` on all the source files.

*   **Package the plugin for release**:
    ```bash
    just package-language-plugin-all <VERSION>
    ```
    This command builds and packages the plugin for all supported platforms (Linux, macOS, Windows). Replace `<VERSION>` with the desired version number.
