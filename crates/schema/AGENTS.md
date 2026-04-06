# Agent Instructions for `schema`

## Purpose

The `schema` crate **reads and parses Pulumi provider schemas** - the JSON/YAML files that describe what resources and functions a provider offers, what properties they have, and what types they use.

**What it does:** Transforms provider schemas from JSON/YAML into structured Rust types that code generators and other tools can work with.

## Architecture Concepts

### Provider Schemas
Every Pulumi provider (AWS, Azure, Google Cloud, etc.) publishes a schema that describes its API. This includes:
- Resources you can create (buckets, VMs, databases)
- Functions you can invoke (get availability zones, lookup data)
- The types and properties they accept and return
- Documentation and examples

### Deserialization Pipeline
The crate processes schemas in stages:
1. **Load**: Read JSON or YAML from file or the Pulumi CLI
2. **Deserialize**: Parse into intermediate representations
3. **Transform**: Convert to a clean, usable model
4. **Filter** (optional): Extract only specific modules/namespaces
5. **Index**: Build lookup maps for efficient access

### Type System Representation
Provider schemas describe complex type systems (primitives, arrays, objects, unions, enums, references). This crate models all these types in Rust, handling the nuances like optional properties, discriminated unions, and constant string types.

### Module Filtering
Large providers like AWS have hundreds of resource types. The filtering system lets you extract just the modules you need (e.g., only EC2 and VPC), along with all their transitive type dependencies.

## Key Concepts

- **Package**: Top-level container for a complete provider schema
- **Type**: Enum representing all possible type kinds (primitives, composites, references)
- **ElementId**: Hierarchical identifier for schema elements (e.g., `aws:ec2/instance:Instance`)
- **Resources & Functions**: The operations a provider exposes
- **GlobalTypes**: Custom types defined by the provider (objects, enums)
- **Filtering**: Extracting subsets of large schemas by namespace

## When to Modify

Modify this crate when:
- Adding support for new schema features from Pulumi
- Changing how types are represented in the model
- Fixing schema parsing issues or provider-specific quirks
- Adding new filtering or querying capabilities

## Testing Philosophy

The crate includes property-based testing (via `proptest`) to verify that schema types can be serialized and deserialized correctly. This catches edge cases in the type system representation.

## Integration Points

- **Used by `rust_generator`**: The code generator reads schemas via this crate
- **Depends on Pulumi CLI**: Can fetch schemas directly from installed providers
