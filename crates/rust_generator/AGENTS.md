# Agent Instructions for `rust_generator`

## Overview

The `rust_generator` crate (`pulumi_gestalt_generator`) is the **Pulumi Gestalt Codegen library** - a sophisticated code generation system that transforms Pulumi provider schemas into idiomatic Rust code.

**Description:** Pulumi Gestalt Codegen library

## Key Modules

### `lib.rs` - Entry Point
- `generate_rust(package, result_path)` - Main code generation function

### `code_generation/` - Example Code
- Converts YAML examples to Rust code
- Used for documentation generation

### `model.rs` - Schema Extensions
Trait extensions for schema types:
- `TypeExt` - Maps schema types to Rust types
- `ElementIdExt` - Converts names to Rust identifiers
- `InputPropertyExt/OutputPropertyExt` - Property conversions

### `description.rs` - Documentation Processing
- State machine for processing Markdown/HTML documentation
- Converts YAML examples to Rust examples
- Handles code blocks in multiple languages

### `output/` - Code Generation
Largest module - generates final Rust code:
- `functions/` - Function generation with Handlebars templates
- `resources/` - Resource generation with Handlebars templates
- `types/` - Type definitions (objects, enums)
- `main.rs` - Main library entry point generation

### `utils.rs` - Helper Functions
- `escape_rust_name()` - Escapes Rust keywords
- `sanitize_rust_identifier()` - Converts to valid identifiers
- `reformat_code()` - Formats Rust code using syn/prettyplease

## Type System Mapping

| Pulumi Type | Rust Type |
|-------------|-----------|
| Boolean | `bool` |
| Integer | `i32` |
| Number | `f64` |
| String | `String` |
| Array<T> | `Vec<T>` |
| Object<K,V> | `HashMap<String, V>` |
| Option<T> | `Option<T>` |
| Ref(Type) | `types::TypeName` |
| DiscriminatedUnion | `OneOf{N}<T1, T2, ...>` |
| ConstString | `ConstString{NAME}` |

## Code Generation Pipeline

```
Pulumi Package Schema (JSON)
    ↓
Parse/Deserialize (pulumi_gestalt_schema)
    ↓
Generate Combined Code:
    ├── Functions → function_*.rs
    ├── Resources → resource_*.rs
    ├── Types → type_*.rs
    └── Main → main.rs (with includes)
    ↓
Output Rust Provider Crate
```

## Testing

**Test Infrastructure:**
- 100+ feature-gated tests with real provider schemas
- Property-based testing using proptest
- Tests organized in `tests/` directory

**Test Providers:**
- AWS (22 partitions), Azure (13 partitions), GCP (13 partitions)
- Cloudflare, Docker, Random, Kubernetes, etc.

## Dependencies

**Code Generation:**
- `handlebars` - Template engine for resources/functions/types
- `askama` - Template engine for main.rs (Jinja2-like)
- `syn` & `quote` - AST parsing and code generation
- `prettyplease` - Rust code formatting

**Schema Processing:**
- `serde` & `serde_json` - Serialization
- `pulumi_gestalt_schema` - Schema definitions

**String Processing:**
- `convert_case` - Case conversion (Snake, Pascal, etc.)
- `regex` - Pattern matching
- `itertools` - Iterator utilities

## Special Considerations

### Generated Code Characteristics
- Uses `bon` builder pattern for all Args structures
- Handles cyclic types via Box wrapping
- Escapes 60+ Rust keywords automatically
- Deterministic output (UNIX_EPOCH timestamps)

### Development Workflow
1. Modify TypeExt trait for new type handling
2. Update Handlebars templates for output changes
3. Run tests to verify against all providers

### Debugging
- Generated code stored in `tests/output/<provider>/src/generated/`
- Compare against expected outputs
- Use `reformat_code()` for syntax validation
