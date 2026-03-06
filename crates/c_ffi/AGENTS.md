# Agent Instructions for `c_ffi`

## Purpose

The `c_ffi` crate **enables C and C++ programs to use Pulumi Gestalt**. It wraps the Rust engine in a C-compatible API, allowing native code to define infrastructure using Pulumi.

**What it does:** Provides a C foreign function interface (FFI) library for Pulumi Gestalt.

## Architecture Concepts

### FFI Bridge Design
C can't directly use Rust's async, generics, or ownership model. The FFI layer bridges this gap by:
- Exposing simple C functions with clear lifecycle
- Managing Rust's async runtime internally
- Using opaque pointers for Rust objects
- Converting between C strings and Rust types
- Making blocking APIs from async operations

### Context-Owned Resources
The `PulumiContext` owns a Tokio runtime and tracks all created outputs. This ensures proper cleanup and prevents memory leaks - when you destroy the context, everything it created is cleaned up automatically.

### Callback-Based Transformations
C programs can transform output values by providing callback functions. The FFI layer marshals data through JSON - C callbacks receive JSON strings and return JSON strings. This keeps the interface simple while supporting complex transformations.

### Memory Management Contract
The FFI has clear ownership rules:
- Context owns all outputs created within it
- Caller owns returned strings and config values (must free them)
- Callbacks receive temporary data (don't store pointers)
- Never access outputs after destroying their context

## Key Concepts

- **PulumiContext**: Main execution context (create → use → destroy)
- **Opaque handles**: Pointers to Rust objects (CustomOutputId, etc.)
- **JSON marshaling**: All values flow through JSON for simplicity
- **Blocking operations**: C sees synchronous APIs, Rust does async internally
- **Header generation**: `cbindgen` auto-generates C headers from Rust

## When to Modify

Modify this crate when:
- Adding new Pulumi operations to expose to C
- Changing error handling (currently panics on errors)
- Improving memory safety or documentation
- Supporting new C/C++ build systems or platforms
- Fixing FFI boundary issues

## Testing Philosophy

Integration tests use an actual C program (`examples/c/`) that exercises the FFI:
- Resource creation and invocation
- Output transformation via callbacks
- Configuration access
- Memory lifecycle correctness

The C example is built with CMake and linked against the generated library.

## Integration Points

- **Wraps `rust_integration`**: Uses the high-level Rust API internally
- **Generates headers**: `cbindgen` creates `pulumi_gestalt.h` from Rust code
- **Used by C/C++ projects**: Link against the shared/static library
