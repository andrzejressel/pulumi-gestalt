# Agent Instructions for `rust_integration`

## Overview

The `pulumi_gestalt_rust_integration` crate provides **Rust helper functions for Pulumi Gestalt** - a high-level async API for integrating with Pulumi's infrastructure platform.

**Description:** Rust helper functions for Pulumi Gestalt

## Key Modules

### `engine.rs` - Core Integration

**Core Types:**
- `Context<T>` - Central struct managing Pulumi execution context
- `Output<T>` - Asynchronous value in the resource graph
- `RegisterResourceOutput<T>` - Output from resource registration
- `ConfigValue<T>` - Configuration values (PlainText or Secret)

**Context Methods:**
- `new()` - Creates context by reading Pulumi environment variables
- `add_output()` - Exports output for stack output
- `register_resource()` - Creates infrastructure resources
- `invoke_resource()` - Calls resource functions/data sources
- `create_native_function_node()` - Creates transformation node
- `create_combine_outputs()` - Combines multiple outputs
- `get_config_value()` - Retrieves configuration values
- `finish()` - Processes pending function requests

### `finish.rs` - Execution Utility
- `finish_lambdas_sequentially()` - Main execution loop
- Polls engine for pending requests
- Executes them sequentially
- Continues until all work completes

### `lib.rs` - Public API
- `get_schema()` - Retrieves provider schema from Pulumi registry

## Integration Patterns

### Basic Resource Creation
```rust
let ctx = Context::new().await;
let output = ctx.create_output(json!(16), false);

let req = RegisterResourceRequest {
    r#type: "random:index/randomString:RandomString".to_string(),
    name: "my_resource".to_string(),
    version: "4.15.1".to_string(),
    inputs: HashMap::from([("length".into(), output)]),
};

let result = ctx.register_resource(req).await;
```

### Output Transformations
```rust
let doubled = output.map(Box::new(|v| {
    let i = v.as_i64().unwrap();
    (i * 2).into()
})).await;
```

### Program Lifecycle
```rust
#[tokio::main]
async fn main() {
    let ctx = Context::new().await;
    // Define resources
    generate_resources(&ctx).await;
    // Process all pending work
    finish_lambdas_sequentially(&ctx).await;
}
```

## Dependencies

**Key Dependencies:**
- `pulumi_gestalt_core` - Low-level engine
- `pulumi_gestalt_domain` - Domain types
- `pulumi_gestalt_grpc_connection` - gRPC bridge
- `pulumi_gestalt_schema` - Schema fetching
- `tokio` - Async runtime (1.45.0+)
- `futures` - Async utilities
- `serde_json` - JSON handling
- `anyhow` - Error handling

## Special Considerations

### Async-First Design
- All operations are async, require tokio runtime
- Use `#[tokio::main]` for entry points

### Environment Variables
Required environment variables (set by Pulumi CLI):
- `PULUMI_ENGINE` - gRPC endpoint for engine
- `PULUMI_MONITOR` - gRPC endpoint for monitor
- `PULUMI_STACK` - Current stack name
- `PULUMI_PROJECT` - Current project name
- `PULUMI_DRY_RUN` - Preview mode flag

### Generic Type Parameter
- `T` represents function context type
- For simple programs, use `Box<dyn Fn(Value) -> Value>`

### Arc<Mutex> Wrapping
- Engine wrapped in `Arc<Mutex>` for shared ownership
- Thread-safe concurrent access
- Synchronization overhead to consider

### Secret Handling
- Configuration distinguishes plaintext and secret types
- Always use `ConfigValue::Secret` for sensitive data
