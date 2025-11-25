# AI Agent Instructions for Pulumi Gestalt

This file contains global instructions for AI agents working with the `pulumi-gestalt` repository. For instructions specific to a particular crate or component, please refer to the `AGENTS.md` file within that directory.

## Project Overview

**Pulumi Gestalt** enables any programming language to work with Pulumi through WebAssembly, C FFI, and native Rust APIs. It automatically generates language bindings from provider schemas and provides runtime support for resource management.

## Environment Setup

The development environment is managed by `devenv`.

1.  **Activate environment** (only on Linux/Unix systems):
    ```bash
    devenv shell
    ```
2.  **Install Rust tools**:
    ```bash
    just install-requirements
    ```

## Development Commands

Commands are run using `just`. Run `just --list` for all available commands.

### Core Commands

*   **Build/format/test all components** (VERY EXPENSIVE - avoid during active development):
    ```bash
    just
    ```
*   **Format source code**:
    ```bash
    just fmt
    ```
*   **Check for issues** (fast, run frequently):
    ```bash
    just check
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

*   **Run tests** (recommended for most development):
    ```bash
    just test
    ```
*   **Run all tests including over 80 example projects** (VERY EXPENSIVE - CI only):
    ```bash
    just test-all
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

## Development Workflow

**Standard workflow for most changes:**
1. Make changes to Rust code
2. Update tests if needed
3. Run `just check` and `just fmt`
4. Run `just test` (covers most cases)
5. Regenerate code if schema changes: `just regenerator`

**Extended workflow for major changes:**
6. Test integration with example projects: `just test-examples`
7. Run target-specific tests if needed: `just test-c` or `just test-native`

## Troubleshooting

**Quick fixes for common issues:**
- Build problems: `just clean && just check`
- Environment issues: Exit and re-run `devenv shell`
- Test failures: Run individual test suites to isolate

# Rust Style Guide for Agents

## Code Formatting
- Max line length: **100 characters**
- Check and fix issues after every change using:
  ```bash
  just check
  just fmt
  ```

## Error Handling

### Always use anyhow context instead of naked `?`

```rust
// Bad
let result = some_operation()?;

// Good
let result = some_operation().context("Failed to perform operation")?;
```

### Use `bail!` instead of `return Err(anyhow!(..))`

```rust
// Bad
return Err(anyhow!("Something went wrong: {}", error_msg));

// Good
bail!("Something went wrong: {}", error_msg);
```

```rust
// Bad
async fn process_task(task: Task) -> Result<(), AgentError> {
    let data = fetch_data(&task.id).await?;
    let processed = transform_data(data)?;
    save_result(processed).await?;
    Ok(())
}

// Good
async fn process_task(task: Task) -> Result<(), AgentError> {
    let data = fetch_data(&task.id)
        .await
        .context("Failed to fetch task data")?;
    
    let processed = transform_data(data)
        .context("Failed to transform data")?;
    
    save_result(processed)
        .await
        .context("Failed to save result")?;
    
    Ok(())
}
```
