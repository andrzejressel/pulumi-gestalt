# Agent Instructions for `grpc`

## Purpose

The `grpc` crate **implements the Pulumi runtime communication protocol**. It's the bridge between Pulumi Gestalt's engine and the actual Pulumi CLI/runtime, translating domain operations into gRPC calls and protocol buffer messages.

**What it does:** Implements the `PulumiConnector` trait using gRPC to talk to the Pulumi runtime over IPC.

## Architecture Concepts

### Protocol Translation
Pulumi's runtime communicates via gRPC using protocol buffers. This crate translates between:
- Clean domain types (from the `domain` crate) ↔ Protobuf messages
- Rust async/await ↔ gRPC streaming protocols
- Pulumi's secret marking format ↔ Domain secret flags

### Two-Level Design
The crate has two abstraction levels:
- **PulumiState**: Lower-level, manages concurrent gRPC tasks with manual polling
- **RealPulumiConnector**: Higher-level, provides clean async API matching the connector trait

Most code uses the connector; the state layer enables advanced concurrent patterns.

### Secret Handling
Pulumi has a specific JSON structure for marking sensitive values. This crate knows how to:
- Wrap secret values when sending to Pulumi (using special signature keys)
- Detect secret markers when receiving from Pulumi
- Preserve the secret flag throughout the conversion process

### Unknown Values
In preview mode, some values don't exist yet. Pulumi represents these with a special UUID. The crate converts between this UUID representation and the domain's `NodeValue::Nothing` enum variant.

## Key Concepts

- **RealPulumiConnector**: The production implementation of `PulumiConnector`
- **Protocol conversion**: Domain types ↔ Protobuf messages
- **Secret wrapping**: Special JSON structure for sensitive data
- **Root stack**: Every Pulumi program has a root Stack resource created automatically
- **Async coordination**: Managing concurrent resource operations via gRPC

## When to Modify

Modify this crate when:
- Pulumi's gRPC protocol changes or adds new operations
- Secret handling requirements change
- Adding support for new Pulumi features (plugins, aliases, etc.)
- Improving error handling or logging for gRPC communication
- Changing how concurrency is managed

## Testing Philosophy

Tests use mock gRPC servers that simulate Pulumi's runtime:
- Verify protocol messages are correctly formatted
- Test concurrent resource operations
- Validate secret and unknown value handling
- Ensure type conversions are bidirectional (round-trip correctly)

The mock servers let tests run without a real Pulumi CLI installation.

## Integration Points

- **Implements `domain::PulumiConnector`**: The production implementation used by the engine
- **Depends on `proto`**: Uses generated gRPC client stubs and protobuf types
- **Used by `rust_integration`**: Provides the real connector for high-level Rust API
