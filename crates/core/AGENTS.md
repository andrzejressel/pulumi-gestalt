# Agent Instructions for `core`

## Purpose

The `core` crate is the **execution engine** that orchestrates infrastructure-as-code programs. It manages the async computation graph that represents resources, data sources, and transformations, coordinating their execution and reporting results to the Pulumi runtime.

**What it does:** Provides the event loop and async task orchestration for executing Pulumi programs written in any language.

## Architecture Concepts

### Event-Driven Async Model
Programs are expressed as directed acyclic graphs of async computations. The engine schedules and executes these computations concurrently, handling dependencies automatically. Language-specific code creates nodes (resources, invocations, transformations), and the engine resolves them in dependency order.

### Output Values
Everything is an `Output<T>` - a shared, lazy async future. This allows multiple parts of the program to depend on the same resource field without duplicating work. Outputs can be transformed, combined, and extracted from, building up the computation graph.

### Two-Way Communication
The engine runs a loop that:
1. Executes pending async tasks (resource creation, provider invocations)
2. Yields control back to the language runtime when custom transformations are needed
3. Receives transformation results and continues execution
4. Completes when all outputs are resolved

This design enables any programming language to integrate with Pulumi Gestalt - they just need to implement the transformation callback mechanism.

### Configuration & Secrets
Configuration is loaded from environment variables at startup. Secret values are wrapped in outputs to defer their evaluation, and the secret flag propagates through the computation graph automatically. This ensures sensitive data is handled correctly throughout the program lifecycle.

## Key Components

- **Engine**: The main orchestrator managing the event loop and computation graph
- **Output**: Lazy async values that can be cloned, transformed, and combined
- **Config**: Configuration loader with secret-aware value handling
- **Preview Mode**: Support for dry-run mode where some values don't exist yet

## When to Modify

Modify this crate when changing:
- How async computations are scheduled and executed
- Output value semantics and transformation behavior
- The contract between the engine and language runtimes
- Configuration loading or secret propagation logic

## Testing Philosophy

Tests use mock Pulumi connectors to verify:
- Computation graph execution order is correct
- Secrets propagate through transformations properly
- Preview mode handles missing values correctly
- The engine properly coordinates with language callbacks

## Integration Points

- **Depends on `domain`**: Uses the `PulumiConnector` trait to communicate with Pulumi runtime
- **Used by language SDKs**: C FFI, WASM runner, and native Rust APIs all build on this engine
- **Coordinates with `grpc`**: The gRPC connector implements the Pulumi communication protocol
