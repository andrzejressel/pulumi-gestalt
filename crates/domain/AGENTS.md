# Agent Instructions for `domain`

## Overview

The `pulumi_gestalt_domain` crate provides core domain models and abstractions for Pulumi Gestalt. It defines the foundational types, traits, and data structures that enable communication between the Pulumi runtime and Rust-based providers.

**Description:** Domain models and abstractions for Pulumi Gestalt

## Key Modules

### 1. Core Types (`lib.rs`)

Provides type-safe wrappers and value representations for resource field management:

- **`FieldName`** - Type-safe wrapper around `String` for field names
  - Prevents accidental string misuse through strong typing
  - Methods: `as_string()` (borrow), `get_inner()` (consume)

- **`ExistingNodeValue`** - Represents an existing resource field value
  - `value: Value` - The actual field value (serde_json)
  - `secret: bool` - Indicates if the value is sensitive data

- **`NodeValue`** - Enum representing field state in the computation graph
  - `Nothing` - Field doesn't exist (used in preview mode)
  - `Exists(ExistingNodeValue)` - Field has an actual value

- **`ResourceFields`** - Container for resource field data
  - `object: HashMap<FieldName, ExistingNodeValue>` - Field values
  - `is_in_preview: bool` - Whether in preview/dry-run mode
  - `get_field_value()` - Returns `Nothing` in preview if missing, `Null` otherwise

### 2. Connector Interface (`connector.rs`)

Defines the async trait and request/response types for Pulumi runtime interaction:

- **`PulumiConnector`** - Core async trait (mockable with `test-utils` feature)
  - `async fn register_resource()` - Create managed resources
  - `async fn resource_invoke()` - Call resource methods
  - `async fn register_outputs()` - Register final stack outputs

- **Request/Response Types** - Built with `bon::Builder` pattern:
  - `RegisterResourceRequest` / `RegisterResourceResult`
  - `ResourceInvokeRequest` / `ResourceInvokeResult`
  - `RegisterOutputsRequest`

## Testing Approaches

Unit tests in `lib.rs` cover `ResourceFields` behavior:
- Field retrieval from HashMap
- Preview mode returns `Nothing` for missing fields
- Actual mode returns `Null` for missing fields

Enable `test-utils` feature for mock support:
```toml
[dev-dependencies]
pulumi_gestalt_domain = { version = "*", features = ["test-utils"] }
```

## Dependencies and Features

**Dependencies:**
- `serde_json` - JSON value handling
- `bon` - Builder pattern support
- `async-trait` - Async trait syntax
- `mockall` (optional) - Mock trait generation

**Features:**
- `default` - Empty feature set
- `test-utils` - Enables mockall for `PulumiConnector` mocking

## Special Considerations

### Preview vs. Actual Mode
- **Preview mode** (`is_in_preview: true`): Missing fields return `NodeValue::Nothing`
- **Actual mode** (`is_in_preview: false`): Missing fields return `NodeValue::Exists(Null)`
- Code must handle both variants appropriately

### Secret Handling
- Check `secret` flag before logging or serializing values
- Implementations should mask or omit secret values from logs

### Builder Pattern Usage
Request types require explicit construction via builder:
```rust
let req = RegisterResourceRequest::builder()
    .name("my-resource".to_string())
    .r#type("aws:s3:Bucket".to_string())
    .object(HashMap::new())
    .version("1.0".to_string())
    .build();
```

### Thread Safety
- `PulumiConnector` requires `Send + Sync`
- All implementations must be safe for concurrent access
