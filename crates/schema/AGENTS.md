# Agent Instructions for `schema`

## Overview

The `pulumi_gestalt_schema` crate is the **Pulumi schema reader** responsible for deserialization, transformation, and manipulation of Pulumi provider schemas from JSON/YAML formats into a structured in-memory model.

**Description:** Pulumi schema reader

## Key Modules

### `lib.rs` - Public API
- `get_schema(provider, version, modules)` - Fetches schema via Pulumi CLI
- `deserialize_package_file(path, filter)` - Reads from JSON/YAML files
- `deserialize_package_json(content, filter)` - Parses JSON strings

### `model.rs` - Core Data Structures
- `Type` enum - All schema types (Boolean, Integer, Number, String, Array, Object, Ref, Option, DiscriminatedUnion, ConstString)
- `InputProperty` / `OutputProperty` / `GlobalTypeProperty` - Property definitions
- `Resource` - Provider resource with input/output properties
- `Function` - Provider function definitions
- `GlobalType` - Custom types (Objects, StringEnum, NumberEnum, IntegerEnum)
- `ElementId` - Unique identifier for schema elements
- `Package` - Top-level container

### `schema.rs` - Deserialization & Transformation
- `to_model()` - Converts deserialized JSON to model::Package
- Type mapping with support for primitives, complex types, unions, enums
- Property handling with required/optional determination

### `filter.rs` - Schema Filtering
- `filter_package()` - Filters resources/functions by module namespace
- Type dependency resolution - retains only used types

### `utils.rs` - Utilities
- `fix_description()` - Corrects documentation for known provider issues
- Docker-specific fixes via embedded markdown

### `arbitrary.rs` - Property-Based Testing (Feature-gated)
- Implements `proptest::Arbitrary` for all model types
- Enables fuzz testing

## Parsing Pipeline

1. Load schema file (JSON/YAML detection)
2. Deserialize into intermediate `schema::Package`
3. Transform via `schema::to_model()` into `model::Package`
4. Optional filtering by module namespace
5. Create lookup maps for efficient access

## Important Types

| Type | Purpose |
|------|---------|
| `Type` | Core type system representation |
| `ElementId` | Hierarchical identifier (namespace/name) |
| `Ref` | Type references (Type, Archive, Asset, Any) |
| `Package` | Top-level schema container |
| `Resource` / `Function` | Provider resources and functions |
| `GlobalType` | Custom types and enums |

## Dependencies

- `serde` / `serde_json` / `serde_yaml` - Serialization
- `anyhow` - Error handling
- `convert_case` - Case conversion
- `itertools` - Iterator utilities
- `pulumi_gestalt_serde_constant_string` - String constants
- `proptest` (optional) - Property-based testing

**Features:**
- `arbitrary` - Enables proptest integration

## Special Considerations

### ElementId Parsing
- Format: `provider:namespace/path:ElementName`
- URL decoding (%2F → /) before namespace extraction
- Special handling for "index" namespace

### Known Provider Issues
- Some providers have incorrect schemas (fields marked required but actually optional)
- `invalid_required_complextype_required_fields()` maintains workaround list
- Example: Docker container fields

### Filter Behavior
- Operates on first namespace component only
- Transitively includes all referenced types
- Preserves global items (empty namespace)

### CLI Dependency
- `get_schema()` requires `pulumi` CLI in PATH
- Sets `PULUMI_AWS_MINIMAL_SCHEMA=true` for AWS to reduce size

## Usage

```rust
use pulumi_gestalt_schema::deserialize_package_file;
use std::path::Path;

// Load schema
let package = deserialize_package_file(
    Path::new("schema.json"),
    None  // No filtering
)?;

// Filter to specific modules
let package = deserialize_package_file(
    Path::new("schema.json"),
    Some(&["ec2", "vpc"])
)?;
```
