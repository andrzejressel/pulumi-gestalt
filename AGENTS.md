# AI Agent Instructions for Pulumi Gestalt

This file contains global instructions for AI agents working with the `pulumi-gestalt` repository. For instructions specific to a particular crate or component, please refer to the `AGENTS.md` file within that directory.

## Environment Setup

The development environment is managed by `devenv`.

1.  **Activate environment**:
    ```bash
    devenv shell
    ```
2.  **Install Rust tools**:
    ```bash
    just install-requirements
    ```

## Development Commands

Commands are run using `just`.

### Core Commands

*   **Build all components**:
    ```bash
    just
    ```
*   **Format source code**:
    ```bash
    just fmt
    ```
*   **Check for issues**:
    ```bash
    just check
    cargo clippy --tests --all-features
    ```
*   **Build Go language plugin**:
    ```bash
    just build-language-plugin
    ```
*   **Build Wasm components**:
    ```bash
    just build-wasm-components
    ```

### Code Generation

*   **Run code generator**:
    ```bash
    just regenerator
    ```
*   **Regenerate generator tests**:
    ```bash
    just regenerate-generator-tests
    ```

## Testing Commands

*   **Run all tests**:
    ```bash
    just test-all
    ```
*   **Run tests with coverage**:
    ```bash
    just test
    ```
*   **Run example tests**:
    ```bash
    just test-examples
    ```
*   **Run C FFI tests**:
    ```bash
    just test-c
    ```
*   **Run native Rust tests**:
    ```bash
    just test-native
    ```

## Documentation Generation

*   **Build and serve documentation**:
    ```bash
    just docs
    ```
*   **Generate Rust documentation**:
    ```bash
    just rust-docs
    ```

## Key Directories

*   `crates/`: Contains all the Rust crates for this project. See the `AGENTS.md` in each crate's directory for more details.
*   `pulumi-language-gestalt/`: Go source for the Pulumi language plugin. See the `AGENTS.md` in this directory for more details.
*   `examples/`: Example projects demonstrating how to use Pulumi Gestalt.
*   `providers/`: Provider schemas and generated code.
*   `proto/`: Protobuf definitions.
*   `docs/`: Source files for the `mkdocs` documentation.
