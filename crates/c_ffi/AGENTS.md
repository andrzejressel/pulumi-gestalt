# Agent Instructions for `c_ffi`

## Overview

The `c_ffi` crate (`pulumi_native_c`) provides a comprehensive C Foreign Function Interface (FFI) for Pulumi Gestalt. It bridges Rust's Pulumi Gestalt infrastructure with C/C++ applications, enabling native code to interact with Pulumi's infrastructure-as-code capabilities.

**Library types:** Dynamic library (`.so`, `.dll`, `.dylib`) and static library

## Key Components

### Core FFI Structures

- **`PulumiContext`** - Main execution context for a Pulumi program
  - Manages Tokio runtime for async operations
  - Tracks all created outputs for lifecycle management
  - Must be paired with `pulumi_destroy_context()` to prevent leaks

- **`CustomOutputId`** - Represents a computed/transformable value
  - Can be mapped, combined, and exported
  - Lifecycle owned by context

- **`CustomCompositeOutputId`** - Represents resource or invoke results
  - Contains structured field data
  - Supports field extraction via `pulumi_composite_output_get_field()`

- **`ConfigValue`** - Tagged enum for configuration values
  - `PlainValue(*mut c_char)` - Plain text configuration
  - `Secret(*mut CustomOutputId)` - Secret value wrapped as output

### Public FFI Functions

**Context Lifecycle:**
- `pulumi_create_context()` - Creates new Pulumi execution context
- `pulumi_finish()` - Finalizes the program, executes pending operations
- `pulumi_destroy_context()` - Deallocates context and tracked outputs

**Output Operations:**
- `pulumi_create_output()` - Creates output with initial value (JSON)
- `pulumi_output_map()` - Transforms output using C callback
- `pulumi_output_combine()` - Combines multiple outputs into array
- `pulumi_output_add_to_export()` - Exports output to stack results

**Resource Operations:**
- `pulumi_register_resource()` - Creates resource (blocks until complete)
- `pulumi_invoke_resource()` - Invokes provider function/data source
- `pulumi_composite_output_get_field()` - Extracts field from result

**Configuration:**
- `pulumi_config_get_value()` - Retrieves configuration values
- `pulumi_config_free()` - Deallocates configuration value

**Schema:**
- `pulumi_get_schema()` - Retrieves protobuf-encoded provider schema
- `pulumi_string_free()` - Deallocates returned strings

## Build Considerations

### Header Generation

The `build.rs` script uses `cbindgen` to auto-generate `pulumi_gestalt.h`:
- Parses Rust source for `#[no_mangle]` functions
- Generates C declarations with renames (types prefixed with `pulumi_`)
- Creates include guards and C++ compatibility

### Library Output

Platform-specific libraries:
- **Linux:** `libpulumi_native_c.so` / `libpulumi_native_c.a`
- **Windows:** `pulumi_native_c.dll` / `pulumi_native_c.lib`
- **macOS:** `libpulumi_native_c.dylib` / `libpulumi_native_c.a`

### Linking from C/C++

Example CMake:
```cmake
include_directories(path/to/crates/c_ffi)
target_link_libraries(program pulumi_native_c)
```

## Testing Approaches

Integration tests via C example project (`/examples/c/`):
- Tests resource creation, invocation, output operations
- Tests configuration access (default and namespaced)
- Validates callback mechanisms
- CMake builds executable with generated headers

## Dependencies

**Core:**
- `pulumi_gestalt_rust_integration` - High-level Rust APIs
- `pulumi_gestalt_schema_protobuf` - Schema serialization
- `tokio` - Async runtime
- `serde_json` - JSON serialization
- `libc` - C FFI primitives
- `anyhow` - Error handling

**Build:**
- `cbindgen` - Automatic C header generation

## Special Considerations for C/C++ Developers

### Memory Management
- **Context owns all outputs** created within it
- Caller owns `ConfigValue` and `CFFIString` returned objects
- Output pointers invalid after context destruction
- Never store output pointers in global/static variables

### Type Conversions
- **JSON serialization** - All values flow through JSON
- Output values must be valid JSON
- Mapping functions receive/return JSON strings

### Callback Constraints
Mapping functions must:
- Return malloc'd C strings (library frees them)
- Never call Pulumi FFI recursively
- Be thread-safe if contexts shared across threads

### Async Execution
All operations synchronous from C's perspective:
- `pulumi_register_resource()` blocks until complete
- Tokio runtime owned by context
- Multiple contexts can coexist with separate runtimes

### Error Handling
Current FFI uses `unwrap()` in many places:
- Errors in conversions panic
- JSON parsing failures panic
- Panics in FFI abort the process
- Production code should add error codes

### Thread Safety
- `PulumiContext` is NOT Send/Sync (uses Rc, RefCell)
- Each thread needs its own context
- No shared context access without external synchronization

### Configuration Namespacing
- NULL name parameter uses project's default namespace
- Non-NULL names access custom configuration namespaces
- Secret values wrapped in CustomOutputId

## Related Crates

- `pulumi_gestalt_rust_integration` - Rust APIs this FFI wraps
- `pulumi_gestalt_schema_protobuf` - Schema codec
- `examples/c` - Reference C implementation
