# Agent Instructions for `serde_constant_string`

## Purpose

The `serde_constant_string` crate provides a **declarative macro for type-safe constant strings**. It creates zero-sized types that serialize to a specific string value, only deserialize from that exact value, and integrate with Pulumi value conversion traits.

**What it does:** Generates types that enforce string constants in serde serialization/deserialization and Pulumi value conversion.

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

### Pulumi Model Integration
The macro also generates `pulumi_gestalt_model` conversion implementations:
- `ToPulumiValue` always produces a string `PulumiValue` with the constant
- `FromPulumiValue` only accepts a matching string `PulumiValue`
- This allows seamless use with `#[derive(ToPulumiValue)]` and
  `#[derive(FromPulumiValue)]` on containing structs

### Use in Discriminated Unions
Perfect for `#[serde(untagged)]` enums where you need to distinguish variants by a constant field value. Each variant gets its own type for the discriminator field.

## Key Concepts

- **generate_string_const!** macro: Creates a zero-sized type for a constant string
- **Zero-sized type**: No memory overhead, pure type safety
- **Compile-time enforcement**: Wrong values cause compilation errors, not runtime errors
- **Serde integration**: Seamless serialization/deserialization
- **Pulumi conversion integration**: Works with `FromPulumiValue` and `ToPulumiValue`
- **Macro hygiene via `__private`**: Uses crate re-exports for stable generated paths

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
- Converts to Pulumi values as exact constant strings
- Rejects non-matching Pulumi string values on conversion
- Zero-sized property (compile-time check)

## Integration Points

- **Used by `schema`**: For type discriminators in schema model
- **Used by `rust_generator`**: Generated code uses these types
- **Works with `model_macros`**: Supports structs deriving `FromPulumiValue` / `ToPulumiValue`
- **Re-exported by `rust`**: Available to Pulumi programs
