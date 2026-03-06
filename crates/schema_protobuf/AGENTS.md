# Agent Instructions for `schema_protobuf`

## Overview

The `schema_protobuf` crate converts Pulumi schema models to and from Protocol Buffer format, acting as a bridge between the Pulumi schema representation and efficient binary serialization.

**Description:** Pulumi schema returned as protobuf

## Public API

- `convert_to_protobuf(package: &Package) -> Result<Vec<u8>>` - Converts Package to binary protobuf

## Key Modules

### `lib.rs` - Main Entry Point
Public function wrapping converter module

### `converter.rs` - Core Conversion Logic
**Main Functions:**
- `package_to_proto()` - Converts Rust Package to protobuf
- `proto_to_package()` - Reverse conversion (test-only)
- Type converters for all schema elements

**Type Mappings:**
| Rust | Protobuf |
|------|----------|
| `Package` | `pulumi::Package` |
| `Resource` / `Function` | `pulumi::Resource` / `Function` |
| `Type` enum | `pulumi::Type` oneof |
| `GlobalType` | `pulumi::GlobalType` |

## Testing

**Integration Tests:**
- 90+ tests with real provider schemas
- Property-based testing with proptest for round-trip verification
- Providers: AWS (22 chunks), Azure (14 chunks), GCP (13 chunks)

**Test Process:**
1. Load schema file
2. Deserialize to Rust Package
3. Convert to protobuf
4. Write binary to `tests/output/{provider}/package.pb`
5. Verify against reference files

## Dependencies

- `pulumi_gestalt_schema` - Rust schema model
- `pulumi_gestalt_proto` (with `pulumi_gestalt` feature) - Protobuf bindings
- `prost` - Protocol Buffer serialization
- `anyhow` - Error handling
- `proptest` (dev) - Property-based testing

## Special Considerations

### Round-Trip Verification
- Proto-to-model converters are test-only (`#[cfg(test)]`)
- Crate is one-way (Rust → Proto) in production
- Tests verify lossless conversion

### Adding New Type Variants
1. Add variant to `proto/pulumi_gestalt.proto` Type oneof
2. Add case in `type_to_proto()` match
3. Add case in `proto_to_type()` match (test-only)
4. Add test case

### Error Handling
- Uses `anyhow::Result<T>` consistently
- Add context at each conversion level
- `.context("Failed to convert X to proto")` pattern
