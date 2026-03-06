# Agent Instructions for `rust_generator`

## Purpose

The `rust_generator` crate **generates idiomatic Rust code from Pulumi provider schemas**. It's the bridge that transforms a provider's JSON schema into type-safe Rust APIs that developers can use in their infrastructure code.

**What it does:** Reads provider schemas and outputs complete Rust crates with resources, functions, and types.

## Architecture Concepts

### Code Generation Philosophy
The generator produces Rust code that feels native - using `Option<T>` for optional fields, `Vec<T>` for arrays, builder patterns for complex construction, and proper documentation. It doesn't just wrap the provider API; it makes it idiomatic Rust.

### Template-Based Generation
The crate uses two templating approaches:
- **Handlebars templates** for repetitive patterns (resources, functions, types)
- **Askama templates** for more complex logic (main library file)

This separation allows easy modification of generated code structure without touching the core logic.

### Type System Mapping
Pulumi schemas describe types in provider-specific ways. The generator maps these to appropriate Rust types (primitives to primitives, arrays to `Vec`, objects to structs, unions to enums). It handles special cases like discriminated unions, constant string types, and cyclic type references.

### Provider Validation
The crate includes 100+ tests that generate code for real providers (AWS, Azure, GCP, Kubernetes, etc.). This validates that the generator handles the full diversity of schema patterns in the Pulumi ecosystem.

## Key Concepts

- **Schema → Rust mapping**: Transforming provider types to idiomatic Rust
- **Builder pattern**: All complex types get builder-based construction
- **Handlebars templates**: Define structure of generated code
- **Code formatting**: Generated code is properly formatted and readable
- **Documentation conversion**: Schema docs → Rust doc comments, YAML examples → Rust examples

## When to Modify

Modify this crate when:
- Adding support for new Pulumi schema features
- Changing the structure or style of generated code
- Fixing code generation bugs for specific providers
- Improving documentation or example generation
- Updating to new Rust idioms or patterns

## Testing Philosophy

The test suite generates code for 100+ provider schemas and verifies:
- Generated code compiles successfully
- Output is deterministic (same input produces same output)
- All provider schema patterns are handled
- Edge cases like cyclic types work correctly

Tests are feature-gated - run specific providers or all of them.

## Integration Points

- **Depends on `schema`**: Reads parsed provider schemas
- **Used by `rust_build`**: Provides the generation API for build scripts
- **Generates code for**: Developers using Pulumi Gestalt with Rust
