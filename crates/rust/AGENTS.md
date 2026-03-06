# Agent Instructions for `rust`

## Overview

The `pulumi_gestalt_rust` crate provides **UNOFFICIAL Rust Pulumi support** built on Pulumi Gestalt. It offers a high-level API that abstracts the complexity of the lower-level integration crate, making it easy for Rust developers to write infrastructure-as-code programs.

**Description:** UNOFFICIAL Rust Pulumi support based on Pulumi Gestalt

## Key Modules

### `lib.rs` - Main Entry Point
- `run()` - Main entrypoint for executing Pulumi programs
- `add_export()` - Exports outputs to stack outputs
- `include_provider!()` macro - Loads generated provider code

### `native.rs` - Core Types
- `Context` - Manages Pulumi execution context
- `Output<T>` - Generic wrapper for Pulumi outputs with type safety
- `CompositeOutput` - Resource outputs with field access
- `ConfigValue<T>` - Configuration values (PlainText or Secret)
- Request types: `RegisterResourceRequest`, `InvokeResourceRequest`

### `macros.rs` - Convenience Macros
- `pulumi_format!` - Creates formatted Output<String> from multiple outputs (up to 16 args)
- `pulumi_combine!` - Combines multiple Output values into tuple Output (up to 16 args)
- `ToOutput` trait - Helper for value conversion

### `input_or_output.rs` - Flexible Input
- `InputOrOutput<T>` enum - Allows either static values or Outputs
- Used by generated provider code for flexible APIs

### `oneof.rs` - Discriminated Unions
- `OneOf2<A, B>`, `OneOf3<A, B, C>`, `OneOf4<A, B, C, D>` - Union types
- Uses `#[serde(untagged)]` for untagged serialization

## Public API

**Entry Points:**
```rust
pulumi_gestalt_rust::run(|ctx| { ... })
pulumi_gestalt_rust::add_export(name, &output)
pulumi_gestalt_rust::include_provider!(provider_name)
```

**Core Types:**
- `Context`, `Output<T>`, `CompositeOutput`, `ConfigValue<T>`
- `RegisterResourceRequest`, `InvokeResourceRequest`
- `InputOrOutput<T>`, `OneOf2/3/4`

## Dependencies

**Core:**
- `anyhow` - Error handling
- `serde` / `serde_json` - Serialization
- `tokio` - Async runtime
- `bon` - Builder pattern support
- `pulumi_gestalt_rust_integration` - Lower-level runtime
- `pulumi_gestalt_serde_constant_string` - Constant string support

## Special Considerations

### Type Safety
- `Output<T>` provides compile-time type safety for async values
- All values must implement `Serialize`

### Async Runtime
- Each `Context` manages its own `tokio` runtime
- Outputs hold `Rc<Runtime>` for synchronous operations
- Uses `block_on()` to bridge async/sync code

### Macro Limitations
- `pulumi_format!` and `pulumi_combine!` support max 16 arguments
- Enforced at compile time

### Secret Handling
- `Context::new_secret()` marks values as sensitive
- Configuration can return `ConfigValue::Secret` wrapping an Output

### Provider Integration
- Use `include_provider!()` macro to include generated code
- Generated from `pulumi_gestalt_build::generate(provider, version)`

## Usage Example

```rust
fn pulumi_main(context: &Context) -> Result<()> {
    let length: Output<i32> = context.new_output(&12).map(|i: i32| i * 3);

    let random_string = random_string::create(
        context,
        "test",
        RandomStringArgs::builder().length(length).build_struct(),
    );

    let formatted = pulumi_format!(&context, "Values: {}", random_string);
    add_export("output", &formatted);
    Ok(())
}
```
