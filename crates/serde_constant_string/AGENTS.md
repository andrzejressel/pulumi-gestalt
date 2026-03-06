# Agent Instructions for `serde_constant_string`

## Overview

This crate provides a procedural macro for generating constant string types that integrate with Serde serialization and deserialization, enabling type-safe enforcement of specific string values.

**Description:** Const string values for serde

## Main Macro

### `generate_string_const!`

**Syntax:**
```rust
generate_string_const!(StructName, "constant_value");
```

**Generates:**
- Zero-sized struct
- `Serialize` - Always serializes to the constant string
- `Deserialize` - Only accepts the exact constant string
- Derives: `Debug`, `PartialEq`, `Eq`, `Copy`, `Clone`, `Hash`, `Default`

**Example:**
```rust
use pulumi_gestalt_serde_constant_string::generate_string_const;
generate_string_const!(ObjectType, "object");

#[derive(Serialize, Deserialize)]
struct Schema {
    #[serde(rename = "type")]
    schema_type: ObjectType,
}
```

## Usage

### Schema Discrimination
Useful for discriminating union types:
```rust
generate_string_const!(ObjectType, "object");
generate_string_const!(ArrayType, "array");

#[derive(Deserialize)]
#[serde(untagged)]
enum Schema {
    Object(ObjectSchema),  // Has type: ObjectType
    Array(ArraySchema),     // Has type: ArrayType
}
```

## Testing

Comprehensive tests cover:
- Serialization to exact constant
- Successful deserialization with matching string
- Deserialization failure with non-matching strings
- Clear error messages

## Dependencies

- `serde` - Serialization framework
- `serde_json` (dev) - JSON support for tests

## Special Considerations

### Zero-Sized Type Optimization
- No runtime memory overhead
- No heap allocation
- Safe to copy without performance concerns

### Case Sensitivity
- String matching is case-sensitive
- `"Object"` ≠ `"object"`

### Type Uniqueness
- Each macro call creates a distinct type
- Different types cannot be assigned to each other, even with same value

### Error Messages
Deserialization failures provide clear messages:
```
invalid value: string `"unexpected"`, expected the string 'constant_value'
```

### Performance
- Serialization: O(1) - writes constant directly
- Deserialization: O(n) - compares input string
- Memory: 0 bytes per instance

## Related Crates

Used by:
- `pulumi_gestalt_schema` - For type discriminators
- `pulumi_gestalt_rust_generator` - For generated SDK code
- `pulumi_gestalt_rust` - Exported via `__private` for SDK consumers
