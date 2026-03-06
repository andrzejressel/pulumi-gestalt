# Agent Instructions for `wit`

## Overview

The `wit` crate (`pulumi_gestalt_wit`) contains **auto-generated Rust bindings** from WebAssembly Interface Type (WIT) definitions. These bindings enable type-safe communication between WASM components and the Pulumi host runtime.

**Description:** Generated WIT bindings for Pulumi Gestalt

## Code Generation

### Source Files
- `wit/world.wit` - WIT interface definitions
- `wit/deps/` - External WIT dependencies

### Generation Process
**Automatic generation via `build.rs`:**
```rust
wit_bindgen::generate!({ world: "pulumi-wasm" });
```

**Output:** Rust types and traits in `src/lib.rs` (generated at build time)

## WIT Worlds

### `pulumi-wasm` World

**Imports (Host → Guest):**
Functions the WASM component can call:
- `register-resource` - Create infrastructure resource
- `invoke` - Call provider function
- `add-export` - Add stack output
- `log` - Write log messages
- `get-config-value` - Retrieve configuration
- `read-or-register-resource` - Get or create resource

**Exports (Guest → Host):**
Functions the host calls on the component:
- `run` - Main entry point
- `map` - Transform output
- `create-value` - Create output from value
- `finalize-value` - Resolve output
- `combine` - Combine multiple outputs
- `duplicate` - Clone output

## Generated Types

### Core Types
```rust
// Opaque handle to async value
pub type OutputId = u32;

// Resolved output value variants
pub enum OutputValue {
    Object(Vec<(String, OutputValue)>),
    String(String),
    Number(f64),
    Bool(bool),
    Array(Vec<OutputValue>),
    Null,
}

// Resource registration result
pub struct Resource {
    pub id: OutputId,
    pub data: Vec<(String, OutputId)>,
}

// Function invocation result
pub struct InvokeResult {
    pub data: Vec<(String, OutputId)>,
    pub failures: Vec<String>,
}
```

### Request Types
```rust
pub struct RegisterResourceRequest {
    pub type_: String,
    pub name: String,
    pub object: Vec<(String, OutputId)>,
    pub version: String,
}

pub struct ResourceInvokeRequest {
    pub token: String,
    pub args: Vec<(String, OutputId)>,
    pub version: String,
}
```

## Generated Traits

### `Host` Trait
Implement to provide host-side functionality:
```rust
pub trait Host {
    fn register_resource(&mut self, request: RegisterResourceRequest)
        -> wasmtime::Result<Resource>;

    fn invoke(&mut self, request: ResourceInvokeRequest)
        -> wasmtime::Result<InvokeResult>;

    fn add_export(&mut self, name: String, value: OutputId)
        -> wasmtime::Result<()>;

    // ... other import functions
}
```

### `Guest` Trait
Interface for calling into WASM component:
```rust
pub trait Guest {
    fn run(&mut self) -> wasmtime::Result<()>;
}

pub trait GuestOutputInterface {
    fn map(&mut self, id: OutputId, func: OutputId)
        -> wasmtime::Result<OutputId>;

    fn create_value(&mut self, value: OutputValue)
        -> wasmtime::Result<OutputId>;

    // ... other export functions
}
```

## Dependencies

**Build Dependencies:**
- `wit-bindgen` (0.38.0) - WIT binding generator

**No Runtime Dependencies:**
All code is generated at build time; the crate only exports generated types.

## Special Considerations

### Generated Code Warning
**DO NOT MANUALLY EDIT** `src/lib.rs` or any generated files. Changes will be overwritten on next build.

### Updating WIT Definitions
1. Modify `wit/world.wit` or files in `wit/deps/`
2. Run `cargo build` to regenerate bindings
3. Update implementations in `wasm_runner` crate to match

### Version Compatibility
- `wit-bindgen` version pinned to ensure stable generation
- WIT definition changes are breaking changes
- Must coordinate updates across:
  - `wit/world.wit` (interface definition)
  - `pulumi_gestalt_wit` (generated bindings)
  - `pulumi_gestalt_wasm_runner` (host implementation)
  - Language-specific WASM SDKs (guest implementation)

### Type Safety
- WIT enforces type safety at component boundary
- Rust types generated with `#[derive]` traits where applicable
- `OutputId` is opaque handle - implementation detail hidden from WASM

### Naming Conventions
- WIT uses kebab-case: `register-resource`
- Generated Rust uses snake_case: `register_resource`
- Automatic conversion by `wit-bindgen`

## Integration Points

**Used By:**
- `pulumi_gestalt_wasm_runner` - Host implementation
- WASM component SDKs - Guest implementation

**Depends On:**
- `wit/world.wit` - Interface definitions
- `wit-bindgen` - Code generation tool

## Debugging Tips

### Viewing Generated Code
```bash
# Generated code location
target/debug/build/pulumi_gestalt_wit-*/out/
```

### Common Issues
- **Build failures:** Check `wit-bindgen` version compatibility
- **Type mismatches:** Ensure WIT definitions match implementations
- **Missing types:** Verify all referenced types are defined in WIT files

## Related Crates

- `pulumi_gestalt_wasm_runner` - Host-side implementation
- `pulumi_gestalt_wasm_rust` - Rust WASM SDK (guest-side)
