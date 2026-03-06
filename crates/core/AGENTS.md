# Agent Instructions for `core`

## Overview

The `pulumi_gestalt_core` crate provides the **execution engine** for Pulumi Gestalt. It orchestrates the creation and management of cloud infrastructure through an async computation graph model, bridging language-specific resource definitions with Pulumi's runtime infrastructure.

**Description:** Core Pulumi Gestalt implementation

## Architecture

### Design Pattern
Event-driven async engine coordinating concurrent operations:
- Resources created as async futures resolving to outputs
- Functions invoked asynchronously with context-specific callbacks
- Output values tracked and reported to Pulumi
- Task orchestration via event loop

### Execution Model
1. Users create computation nodes (resources, functions, etc.)
2. Each node returns `Output<T>` - a shared, cloneable async future
3. `Engine::run()` processes events (completed tasks, pending function requests)
4. Native functions invoked with context and data, results flow back
5. Final outputs collected and reported to Pulumi runtime

## Key Modules

### `engine.rs` - Core Orchestration Engine

Main orchestrator managing the async computation graph and event loop.

**Key Types:**
- `Engine<FunctionContext>` - Generic over user-defined context type
- `Output<T>` - Shared async future wrapper
- `NativeFunctionRequest<FunctionContext>` - Callback requests
- `ConfigValue` - Enum for plain text or secret config values

**Key Methods:**
- `new(pulumi, config)` - Initialize with connector and configuration
- `add_output()` - Register top-level output for final reporting
- `create_register_resource_node()` - Queue resource creation
- `create_resource_invoke_node()` - Queue resource invocation
- `create_native_function_node()` - Create transformation with callback
- `create_combine_outputs()` - Merge multiple outputs into array
- `create_extract_field()` - Extract field from resource output
- `run()` - Main event loop (async) - returns pending requests or None

**Implementation:**
- Uses `FuturesUnordered` for concurrent task execution
- Channels for native function request queueing
- Atomic flags for single-execution guarantees
- `futures::select!` for event multiplexing

### `config.rs` - Configuration Management

Loads and provides access to Pulumi configuration.

**Key Types:**
- `Config` - Configuration container

**Methods:**
- `from_env_vars()` - Load from environment (recommended)
- `get(namespace, key)` - Retrieve value (namespace defaults to project)

**Environment Variables:**
- `PULUMI_CONFIG` - JSON object of config values
- `PULUMI_CONFIG_SECRET_KEYS` - JSON array of secret keys
- `PULUMI_PROJECT` - Project name (default namespace)

**Concepts:**
- Keys follow format: `{namespace}:{key}` (e.g., `aws:region`)
- Secrets wrapped in `Output<String>` for deferred evaluation

### `model.rs` - Domain Types

Lightweight type definitions:
- `FunctionName` - Newtype wrapper around String for type safety

## Important Types

### Output Types
```rust
pub type RawOutput = Output<NodeValue>;
pub type RegisterResourceOutput = Output<Arc<ResourceFields>>;
pub struct Output<T> { value: Shared<BoxFuture<'static, T>> }
```

**Properties:**
- Cloneable (multiple consumers of same async result)
- Generic over result type T
- Lazy evaluation (futures execute in event loop)

### Configuration Types
```rust
pub enum ConfigValue {
    PlainText(String),
    Secret(RawOutput),
}
```

## Testing Approaches

Comprehensive test coverage using:
- `mockall::MockPulumiConnector` - Mock Pulumi connector
- `tokio::test` - Async test support
- `static_assertions` - Compile-time trait verification

**Test Areas:**
- Output registration and reporting
- Function transformation node behavior
- Array combination with secrets and Nothing values
- Configuration retrieval (plain text, secrets, namespaces)

## Dependencies

**Production:**
- `futures` - Async primitives (select, FuturesUnordered, BoxFuture, Shared)
- `anyhow` - Error handling
- `uuid` - Unique identifiers (v7)
- `serde_json` - JSON value handling
- `pulumi_gestalt_domain` - Core types and connector trait

**Development:**
- `mockall` - Mock trait generation
- `tokio` - Async runtime for tests
- `static_assertions` - Compile-time assertions

## Special Considerations

### Async Execution Model
- `Engine::run()` IS the event loop
- Callers must repeatedly invoke `run()` until it returns `None`
- Between calls, handle returned function requests
- Enables language-specific runtime integration

### Ownership and Cloning
- `Output<T>` uses `Shared<BoxFuture>` for multiple ownership
- All clones share same underlying future (computed once)
- Safe to pass to different threads (Arc-based)

### Context Type Parameter
- `Engine<FunctionContext>` allows arbitrary user context type
- Context captured when creating function nodes via UUID
- Retrieved and returned when function invoked
- Enables language-specific callback patterns

### Secret Handling
- Secrets marked at config load time
- Secret flag propagates through computation graph
- Output array is secret if ANY input is secret
- Secrets remain JSON values - encryption by Pulumi runtime

### Preview Mode Support
- `NodeValue::Nothing` represents preview-mode missing values
- Any combine touching `Nothing` returns `Nothing`
- Proper handling of uncertain preview values

### Thread Safety
- `Engine<T>` is Send + Sync (verified by static assertions)
- Can be wrapped in Arc and shared across threads
- Interior mutability via Mutex protects internal state

## Usage Patterns

### Basic Resource Creation
```rust
let resource_output = engine.create_register_resource_node(
    "aws:s3/bucket:Bucket".to_string(),
    "my-bucket".to_string(),
    inputs_map,
    "4.0.0".to_string()
);
```

### Main Loop
```rust
loop {
    match engine.run().await {
        Some(request) => {
            let result = invoke_user_function(&request.context, &request.data);
            request.return_mailbox.send(result)?;
        }
        None => break,
    }
}
```

## Related Crates

- `pulumi_gestalt_domain` - Domain types and Pulumi connector interface
- Generated language bindings - Use this engine via public API
