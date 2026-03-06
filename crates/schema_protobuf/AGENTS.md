# Agent Instructions for `schema_protobuf`

## Purpose

The `schema_protobuf` crate **converts provider schemas to protobuf format**. This enables efficient binary serialization of schemas for transmission over FFI boundaries or network protocols.

**What it does:** Transforms in-memory schema models to protocol buffer binary format.

## Architecture Concepts

### Binary Schema Transmission
Sometimes you need to send a provider schema across a boundary where JSON isn't ideal:
- C FFI (transferring to C/C++ code)
- Network protocols
- Efficient storage

Protobuf provides a compact, versioned binary format that's faster and smaller than JSON.

### One-Way Conversion
The crate focuses on Rust → Protobuf conversion. The reverse conversion (Protobuf → Rust) exists only in tests to verify correctness. In production, schemas flow from the schema reader to protobuf, not back.

### Round-Trip Testing
The crate includes extensive tests with 90+ real provider schemas, verifying that:
- Every schema element converts correctly
- No data is lost in conversion
- The reverse conversion matches the original (for test-only validation)

## Key Concepts

- **convert_to_protobuf**: Main API that converts Package to binary bytes
- **Round-trip verification**: Tests ensure lossless conversion
- **Real provider testing**: Validates against actual AWS, Azure, GCP schemas
- **Binary output**: Returns `Vec<u8>` for efficient transmission

## When to Modify

Modify this crate when:
- Schema model adds new type variants or fields
- Protobuf schema definition changes
- Need to optimize serialization performance
- Adding new schema features that need protobuf representation

## Testing Philosophy

Testing is comprehensive with real providers:
- Load real schema files
- Convert to protobuf
- Convert back to Rust (test-only)
- Verify they match
- Save binary output for reference

This catches any conversion bugs with real-world schema complexity.

## Integration Points

- **Uses `schema`**: Converts its Package type
- **Uses `proto`**: Outputs its protobuf types
- **Used by `c_ffi`**: Provides schema to C clients via protobuf
