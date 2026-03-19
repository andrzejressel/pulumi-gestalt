# AI Agent Instructions for Pulumi Gestalt

This file contains global instructions for AI agents working with the `pulumi-gestalt` repository. For instructions specific to a particular crate or component, please refer to the `AGENTS.md` file within that directory.

## Project Overview

**Pulumi Gestalt** enables any programming language to work with Pulumi through WebAssembly, C FFI, and native Rust APIs. It automatically generates language bindings from provider schemas and provides runtime support for resource management.

## Environment Setup

The development environment is managed by `mise`.

1. **Install `mise`**: Follow instructions at https://mise.jdx.dev/getting-started.html.
2. **Bootstrap environment**:
    ```bash
    mise install
    just install-requirements
    ```

## Development Commands

Commands are run using `just`. Run `just --list` for all available commands.

### Core Commands

*   **Build/format/test all components** (VERY EXPENSIVE - avoid during active development):
    ```bash
    just
    ```
*   **Never run `--all-features` locally** (CI only):
    Running `cargo test --all-features` or similar commands with `--all-features` can be extremely slow and resource-intensive as it enables all provider generators.
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
*   **IMPORTANT**: Always run `just regenerate-generator-tests` after modifying Handlebars (`.handlebars`) or Jinja (`.jinja`) templates to ensure all generator tests are up-to-date with the latest template changes.

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
- Environment issues: Re-run `mise install`
- Test failures: Run individual test suites to isolate

## Changelog Management

The project uses a structured changelog system located in the `.changelog/` directory.

### Creating a Changelog Entry

When making changes that should be documented in the changelog:

1. **Create a YAML file** in `.changelog/unreleased/` with a descriptive name (e.g., `add_new_feature.yaml`)

2. **Use the following format**:
   ```yaml
   type: <Type>
   title: <Short description>
   description: |
     <Optional longer description>
     Can be multi-line
   ```

3. **Available types**:
   - `Announcement` - Important announcements (shown at the top)
   - `Added` - New features
   - `Changed` - Changes to existing functionality
   - `Deprecated` - Soon-to-be removed features
   - `Removed` - Removed features
   - `Fixed` - Bug fixes
   - `Security` - Security-related changes

4. **Example entries**:
   ```yaml
   type: Added
   title: Added example for Wasm integration
   ```

   ```yaml
   type: Removed
   title: Remove Rust abstraction over Wasm
   description: |
     This change simplifies code and documentation by removing the Rust abstraction layer over Wasm.
   ```

5. **File naming**: Use descriptive names with underscores (e.g., `remove_x86_mac_support.yaml`)

The changelog entries are automatically processed during release to generate the final `CHANGELOG.md` file.

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
