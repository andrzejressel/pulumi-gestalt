# Agent Instructions for `domain`

## Purpose

The `domain` crate defines the **core abstractions and types** that form the contract between different parts of Pulumi Gestalt. It's the shared vocabulary that enables the execution engine, language SDKs, and Pulumi runtime to communicate.

**What it does:** Provides foundational types and the `PulumiConnector` trait that defines how to interact with the Pulumi runtime.

## Architecture Concepts

### Type-Safe Field Management
Resources in Pulumi have fields (properties), and those fields have values. This crate provides typed wrappers (`FieldName`, `NodeValue`, `ResourceFields`) that prevent common mistakes like mixing up field names with other strings or accessing nonexistent fields incorrectly.

### Preview vs Actual Execution
Infrastructure-as-code programs can run in two modes:
- **Preview mode**: A dry-run where you see what *would* happen (some values don't exist yet)
- **Actual mode**: The real execution where resources are created

The domain types model this difference explicitly. Missing fields behave differently in each mode, ensuring programs handle uncertainty correctly.

### Connector Abstraction
The `PulumiConnector` trait defines the operations needed to interact with Pulumi: register resources, invoke provider functions, and report outputs. This abstraction allows multiple implementations (gRPC for production, mocks for testing, etc.) without changing the core engine.

### Secret Tracking
Some configuration values are sensitive (passwords, API keys). The domain types track which values are secrets so they can be handled appropriately throughout the system - never logged, always encrypted when persisted.

## Key Concepts

- **FieldName**: Type-safe wrapper preventing string confusion
- **NodeValue**: Represents field states (exists with a value, or doesn't exist in preview)
- **ResourceFields**: Container for all fields of a resource, with preview-aware field access
- **PulumiConnector**: The async trait defining how to talk to Pulumi runtime
- **Builder Pattern**: Request types use `bon` builders for clear, type-safe construction

## When to Modify

Modify this crate when:
- Adding new operations that need to communicate with Pulumi runtime
- Changing the contract between engine and runtime
- Adding new domain concepts that multiple crates need to understand
- Modifying how preview mode or secrets are represented

## Testing Philosophy

The domain types are simple data structures, so tests focus on behavior:
- Field access returns correct values in preview vs actual mode
- Type safety is enforced at compile time
- Mock connector trait works correctly for testing other crates

Enable the `test-utils` feature to get mock implementations of the `PulumiConnector` trait.

## Integration Points

- **Used by `core`**: The engine depends on these types and the connector trait
- **Implemented by `grpc`**: Provides the real Pulumi runtime connection
- **Used by all language SDKs**: Types flow through FFI boundaries and WASM interfaces
