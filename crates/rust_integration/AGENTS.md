# Agent Instructions for `rust_integration`

## Purpose

The `rust_integration` crate provides **mid-level Rust integration with the Pulumi engine**. It sits between the low-level `core` engine and the high-level `rust` API, offering async-first functions for creating resources and managing execution.

**What it does:** Async Rust API for Pulumi integration, managing the engine lifecycle and resource operations.

## Architecture Concepts

### Context as Central Manager
The `Context` type encapsulates everything needed to run a Pulumi program:
- Connection to Pulumi runtime (via gRPC connector)
- The execution engine
- Configuration
- Async runtime coordination

Programs create a context, use it to define resources, then call `finish()` to execute everything.

### Async-First Design
Everything is async. Resource creation returns immediately with an `Output<T>`, and the actual work happens when you call `finish_lambdas_sequentially()`. This allows:
- Defining all resources upfront
- Parallel execution where possible
- Proper dependency ordering

### Output Transformations
The `Output<T>` type supports transformations (map, combine, extract fields). This crate provides the machinery to create these transformations and execute them when the engine runs.

### Environment-Driven Configuration
The context reads Pulumi environment variables automatically. This means programs don't need command-line arguments or config files - they just work when run by Pulumi.

## Key Concepts

- **Context**: Main manager for a Pulumi program's execution
- **Output<T>**: Async value that can be transformed and combined
- **finish_lambdas_sequentially**: The execution loop that actually runs everything
- **Environment variables**: PULUMI_ENGINE, PULUMI_MONITOR, etc. configure connections
- **ConfigValue**: Distinguishes plain text from secret configuration

## When to Modify

Modify this crate when:
- Adding new operations to the integration layer
- Changing how the engine lifecycle is managed
- Modifying the async execution model
- Adding convenience methods for common patterns
- Supporting new Pulumi runtime features

## Testing Philosophy

Tests verify the integration layer works correctly:
- Context creation from environment
- Resource registration flows through to engine
- Configuration loading works
- Outputs can be created and transformed

Most complex logic is in `core`, so these tests focus on the integration glue.

## Integration Points

- **Wraps `core`**: Uses the execution engine internally
- **Uses `grpc`**: Connects to Pulumi runtime via gRPC
- **Wrapped by `rust`**: The high-level API builds on this
- **Used by `c_ffi`**: The C FFI uses this for its implementation
