# Agent Instructions for `rust_build`

## Overview

The `pulumi_gestalt_build` crate is a **codegen module for Pulumi Gestalt** - a build-time code generation tool that automatically generates Rust bindings for Pulumi providers from their schemas.

**Description:** Codegen module for Pulumi Gestalt

## Public API Functions

| Function | Purpose | Usage |
|----------|---------|-------|
| `generate(provider, version)` | Generate from Pulumi registry | `pulumi_gestalt_build::generate("random", "4.15.0")?` |
| `generate_with_filter(provider, version, modules)` | Generate specific modules | `generate_with_filter("aws", "6.0.0", &["ec2", "s3"])?` |
| `generate_from_schema(path)` | Generate from local schema file | `generate_from_schema(Path::new("schema.json"))?` |
| `generate_from_schema_with_filter(path, modules)` | Generate from local schema with filtering | Both schema file and module filtering |

## How to Use in build.rs

### Basic Usage
```rust
// build.rs
fn main() -> Result<(), Box<dyn Error>> {
    pulumi_gestalt_build::generate("github", "5.26.0")?;
    Ok(())
}
```

### With Module Filtering
```rust
fn main() -> Result<(), Box<dyn Error>> {
    pulumi_gestalt_build::generate_with_filter(
        "aws", "6.0.0", &["ec2", "s3", "rds"]
    )?;
    Ok(())
}
```

### Using Local Schema
```rust
fn main() -> Result<(), Box<dyn Error>> {
    pulumi_gestalt_build::generate_from_schema(
        Path::new("../provider.json")
    )?;
    Ok(())
}
```

## Integration with lib.rs

```rust
// lib.rs
mod provider {
    include_provider!("provider_name");
}

use provider::resource::ResourceArgs;
```

## Dependencies

- `pulumi_gestalt_schema` - Schema retrieval and deserialization
- `pulumi_gestalt_generator` - Actual code generation logic
- `anyhow` - Error handling
- `tempfile` - Temporary file handling

## Special Considerations

### CLI Dependency
- `generate()` requires `pulumi` CLI installed and in PATH
- Sets `PULUMI_AWS_MINIMAL_SCHEMA=true` for AWS to reduce schema size

### Module Filtering
- Module names must match exactly as in Pulumi Registry
- Use filtering for large providers (especially AWS) to reduce code size and compilation time

### Best Practices
- Pin provider versions for reproducible builds
- Use module filtering for large providers
- Consider separating generated code into a different crate for large providers

### Generated Output
Code placed in: `{OUT_DIR}/pulumi/{provider_name}/`

## Related Crates

- `pulumi_gestalt_schema` - Schema model
- `pulumi_gestalt_generator` - Code generation logic
- `pulumi_gestalt_rust` - Runtime support
