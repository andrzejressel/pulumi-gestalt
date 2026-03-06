# Agent Instructions for `rust_build`

## Purpose

The `rust_build` crate **integrates code generation into Rust build scripts**. It provides simple functions you call from `build.rs` to automatically generate provider bindings at compile time.

**What it does:** Build-time code generation API for including Pulumi providers in Rust projects.

## Architecture Concepts

### Build Script Integration
Rust's `build.rs` runs before compilation. This crate provides functions that:
- Fetch provider schemas (from Pulumi registry or local files)
- Generate Rust code using the generator
- Write output to `OUT_DIR` where it can be included in your crate

This means provider bindings are always up-to-date and match your declared versions.

### Two Schema Sources
You can generate from:
1. **Pulumi registry**: Fetch the schema automatically by provider name and version
2. **Local file**: Use a schema file you have locally (useful for custom providers or offline development)

### Module Filtering
Large providers like AWS have thousands of resource types. You can filter to specific modules (e.g., only S3 and EC2) to:
- Reduce generated code size
- Speed up compilation
- Make IDE autocomplete more manageable

## Key Concepts

- **build.rs integration**: Called from build scripts, not regular code
- **Automatic schema fetching**: Can download schemas from Pulumi registry
- **Code generation**: Wraps the `rust_generator` crate
- **Module filtering**: Extract only needed parts of large providers
- **OUT_DIR placement**: Generated code goes where Rust expects it

## When to Modify

Modify this crate when:
- Changing how schemas are fetched or cached
- Adding new code generation options (e.g., feature flags, customization)
- Supporting new schema sources
- Improving build script error messages or diagnostics

## Testing Philosophy

Testing happens at two levels:
- Unit tests verify API surface and parameter handling
- Integration tests in example projects verify generated code compiles and works

Most validation comes from the generator tests, which this crate wraps.

## Integration Points

- **Calls `schema`**: Fetches and parses provider schemas
- **Calls `rust_generator`**: Performs the actual code generation
- **Used in `build.rs`**: Rust projects call these functions from build scripts
