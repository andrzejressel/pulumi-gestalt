# Agent Instructions for `serde_constant_string`

## Purpose

The `serde_constant_string` crate provides a **procedural macro for type-safe constant strings**. It creates zero-sized types that serialize to a specific string value and only deserialize from that exact value.

**What it does:** Generates types that enforce string constants in serde serialization/deserialization.

## Architecture Concepts

### Type-Safe String Constants
Sometimes you need a field that must always have a specific string value (like type discriminators in JSON). Instead of using `String` and checking at runtime, this macro creates a type that enforces the value at compile time.

### Zero-Cost Abstraction
The generated types are zero-sized - they take no memory at runtime. The constant string is embedded in the type itself, not stored in each instance. This means you get type safety without any runtime cost.

### Serde Integration
The macro generates proper `Serialize` and `Deserialize` implementations:
- Serialization always outputs the constant string
- Deserialization succeeds only if the input matches exactly
- Clear error messages when deserialization fails

### Use in Discriminated Unions
Perfect for `#[serde(untagged)]` enums where you need to distinguish variants by a constant field value. Each variant gets its own type for the discriminator field.

## Key Concepts

- **generate_string_const!** macro: Creates a zero-sized type for a constant string
- **Zero-sized type**: No memory overhead, pure type safety
- **Compile-time enforcement**: Wrong values cause compilation errors, not runtime errors
- **Serde integration**: Seamless serialization/deserialization

## When to Modify

Modify this crate when:
- Changing the generated code structure
- Adding new derive traits or implementations
- Improving error messages
- Supporting new serde features

This is a small, stable crate that rarely needs changes.

## Testing Philosophy

Tests verify the generated code works correctly:
- Serializes to the exact constant
- Deserializes successfully from matching strings
- Rejects non-matching strings with clear errors
- Zero-sized property (compile-time check)

## Integration Points

- **Used by `schema`**: For type discriminators in schema model
- **Used by `rust_generator`**: Generated code uses these types
- **Re-exported by `rust`**: Available to Pulumi programs
