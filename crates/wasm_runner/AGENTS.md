# Agent Instructions for `wasm_runner`

## Purpose

The `wasm_runner` crate **executes Pulumi programs compiled to WebAssembly**. It's the host runtime that loads WASM components and connects them to the Pulumi engine, enabling any language that compiles to WASM to work with Pulumi Gestalt.

**What it does:** WASM Component Model runtime for Pulumi infrastructure programs.

## Architecture Concepts

### Host-Guest Model
WebAssembly has a host (Rust code running `wasmtime`) and a guest (the WASM component). This crate is the host that:
- Loads and instantiates WASM components
- Provides host functions the guest can call (register resources, invoke functions, etc.)
- Calls guest functions to drive execution
- Bridges between WASM's synchronous model and Pulumi's async model

### WIT-Based Interface
The boundary between host and guest is defined by WIT (WebAssembly Interface Types). This provides:
- Type-safe function calls across the boundary
- Automatic marshaling of data structures
- Clear contract that both sides implement

### Async Bridging
WASM components are synchronous, but Pulumi operations are async. The runner solves this by:
- Making host functions (called by guest) appear synchronous
- Queueing async work during guest execution
- Processing all pending work after the guest returns

### WASI Support
The runner provides WASI (WebAssembly System Interface) support, giving WASM programs access to:
- Standard I/O (stdin/stdout/stderr)
- Environment variables
- File system operations (if needed)

## Key Concepts

- **wasmtime**: The WebAssembly runtime engine
- **WIT bindings**: Type-safe host-guest interface
- **Host functions**: Operations the WASM can call (register_resource, invoke, etc.)
- **Guest functions**: Functions the host calls (run, map, combine, etc.)
- **Async queuing**: Collecting async work while guest runs, processing after

## When to Modify

Modify this crate when:
- WIT interface changes (new operations, changed signatures)
- Adding new WASI capabilities
- Changing async execution model
- Supporting new WASM features or optimizations
- Debugging WASM-specific issues

## Testing Philosophy

Integration tests use actual WASM components (from examples):
- Load real WASM files
- Execute them against mock Pulumi runtime
- Verify correct resource operations
- Test output transformations and combinations

This ensures the host-guest boundary works correctly with real WASM code.

## Integration Points

- **Uses `wit`**: Implements the WIT-defined host interface
- **Uses `rust_integration`**: Wraps the Context API for guest calls
- **Executes WASM programs**: The runtime for WASM-based Pulumi programs
- **Depends on `wasmtime`**: The WASM execution engine
