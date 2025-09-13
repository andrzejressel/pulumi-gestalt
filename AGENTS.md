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

*   **Build/format/test all components** (VERY EXPENSIVE):
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

*   **Run tests**:
    ```bash
    just test
    ```
*   **Run all tests including over 80 examples projects** (VERY EXPENSIVE):
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

* Make changes to Rust code
* Update tests if needed
* Run `just check` and `just fmt`
* Run relevant test suites (`just test` should cover most cases)
* Regenerate code if schema changes: just regenerator
* Test integration with example projects

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
