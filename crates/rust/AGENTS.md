# Agent Instructions for `rust`

## Purpose

The `rust` crate provides the **high-level Rust API for writing Pulumi programs**. It's designed for Rust developers who want a clean, idiomatic way to define infrastructure without dealing with the lower-level engine details.

**What it does:** Offers a friendly Rust API for infrastructure-as-code, wrapping the complexity of the engine and runtime.

## Architecture Concepts

### Type-Safe Outputs
Infrastructure resources are created asynchronously, and their properties aren't known until deployment. The `Output<T>` type captures this - it's a typed future value that you can transform, combine, and export. The type parameter ensures compile-time safety even for values that don't exist yet.

### Context-Managed Execution
The `Context` object manages the execution lifecycle. It owns the async runtime, tracks outputs, and coordinates with the Pulumi engine. Users create resources through the context, and the runtime handles all the async coordination automatically.

### Provider Integration
Generated provider code (from `rust_generator`) integrates seamlessly. The `include_provider!` macro brings in thousands of resource types with full type safety, builder patterns, and documentation - all generated from provider schemas.

### Macro Convenience
Infrastructure code often needs to combine multiple async values (format strings, combine outputs). The `pulumi_format!` and `pulumi_combine!` macros make this ergonomic, handling the async complexity behind simple syntax.

## Key Concepts

- **Context**: The execution manager (create once per program)
- **Output<T>**: Type-safe async values that can be transformed and combined
- **CompositeOutput**: Resource results with field extraction
- **ConfigValue**: Configuration that might be plain text or secret
- **Macros**: `pulumi_format!`, `pulumi_combine!` for convenient async operations
- **include_provider!**: Macro to include generated provider code

## When to Modify

Modify this crate when:
- Adding new convenience features to the Rust API
- Changing how outputs are created or transformed
- Adding new macros for common patterns
- Improving error messages or ergonomics
- Supporting new Pulumi features at the API level

## Testing Philosophy

Tests focus on the developer experience:
- Macros expand correctly
- Output transformations work as expected
- Type conversions are correct
- Integration with generated provider code works

The crate is thin wrapper over `rust_integration`, so most logic is tested there.

## Integration Points

- **Wraps `rust_integration`**: Uses the lower-level integration layer
- **Used with generated providers**: Works with code from `rust_generator`
- **Used by Rust developers**: The public API for writing Pulumi programs in Rust
