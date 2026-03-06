# Agent Instructions for `wasm_runner`

## Overview

The `wasm_runner` crate provides the **WebAssembly Component Model runtime** for executing Pulumi infrastructure code compiled to WASM. It bridges WASM components with the Pulumi engine using the WIT (WebAssembly Interface Types) bindings.

**Description:** Wasm runner for Pulumi Gestalt

## Key Modules

### `lib.rs` - Main Entry Point

**Core Functions:**
- `run()` - Main entry point that executes WASM component
- Creates Context from environment
- Instantiates WASM component
- Calls component's `run()` function
- Processes pending work via `finish_lambdas_sequentially()`

**Execution Flow:**
```
1. Context::new() - Read Pulumi environment variables
2. Load WASM bytes from file
3. Component::from_binary() - Parse WASM component
4. Instantiate with host bindings
5. Call guest.run()
6. finish_lambdas_sequentially() - Process async work
```

### `exports.rs` - WIT Exports Implementation

Implements the `pulumi_wasm.output_interface` world for host → guest calls:

**Exported Functions:**
- `map()` - Transform output value with function
- `create_value()` - Create output from JSON value
- `finalize_value()` - Resolve output to final value
- `combine()` - Combine multiple outputs
- `duplicate()` - Clone an output reference

**OutputInterface Trait:**
All functions receive `OutputId` and return new `OutputId` or results.

### `imports.rs` - WIT Imports Implementation

Implements the `pulumi_wasm.external_world` imports for guest → host calls:

**Imported Functions:**
- `register_resource()` - Create infrastructure resource
- `invoke()` - Call provider function/data source
- `add_export()` - Add stack output
- `log()` - Write to Pulumi logs
- `get_config_value()` - Retrieve configuration
- `read_or_register_resource()` - Get or create resource

**Implementation:**
Converts WIT types to domain types and delegates to `Context<T>`.

## Type System

### Core Types
- `OutputId` - Opaque handle to async output value (u32)
- `OutputValue` - Resolved output (object, string, number, bool, array, null)
- `Resource` - Registered resource with ID and outputs
- `RegistrationResult` - Result from resource registration
- `InvokeResult` - Result from function invocation

### Serialization
- Uses `serde_json` for Value ↔ OutputValue conversion
- Custom From implementations for WIT type conversions

## Integration with WASM Component Model

### Host-Guest Interface
```
┌─────────────────┐
│  WASM Component │  (Guest)
│   (*.wasm)      │
└────────┬────────┘
         │ WIT bindings
         │
┌────────▼────────┐
│  wasm_runner    │  (Host)
│  + wasmtime     │
└────────┬────────┘
         │
┌────────▼────────┐
│ Pulumi Engine   │
│ via gRPC        │
└─────────────────┘
```

## Dependencies

**Core Dependencies:**
- `wasmtime` (27.0.0) & `wasmtime-wasi` - WASM runtime
- `pulumi_gestalt_rust_integration` - Pulumi integration layer
- `pulumi_gestalt_domain` - Domain types
- `pulumi_gestalt_wit` - Generated WIT bindings
- `tokio` - Async runtime

**Utility Dependencies:**
- `serde_json` - JSON serialization
- `anyhow` - Error handling

## Special Considerations

### WASI Preview 2 Support
- Uses `wasmtime-wasi` with WasiCtxBuilder
- Configures stdin/stdout/stderr streams
- Provides filesystem and environment access to WASM

### Async Execution Model
- WASM components are synchronous
- Host functions (imports) bridge to async Rust
- `finish_lambdas_sequentially()` processes async work after component returns

### Memory Management
- OutputId values managed by host
- WASM linear memory isolated from host
- Type conversions at WIT boundary

### Error Handling
- WIT errors mapped to `anyhow::Result`
- Context propagated through error chains
- Failed operations return detailed error messages

### Component Instantiation
- Single-threaded execution model
- Fresh state for each execution
- No persistent state between runs

## Environment Requirements

Same as `pulumi_gestalt_rust_integration`:
- `PULUMI_ENGINE` - gRPC endpoint
- `PULUMI_MONITOR` - gRPC endpoint
- `PULUMI_STACK` - Stack name
- `PULUMI_PROJECT` - Project name
- `PULUMI_DRY_RUN` - Preview mode flag

## Related Crates

- `pulumi_gestalt_wit` - WIT bindings (generated)
- `pulumi_gestalt_rust_integration` - Context and engine
- `pulumi_gestalt_wasm_rust` - SDK for writing WASM components
