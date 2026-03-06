# Agent Instructions for `grpc`

## Overview

The `grpc` crate provides an abstraction layer for gRPC connections in Pulumi Gestalt. It handles all communication with Pulumi's resource monitor and engine services via Tonic gRPC client, bridging the gap between Pulumi Gestalt's domain model and Pulumi's gRPC-based IPC protocol.

**Description:** Abstraction layer for gRPC connections in Pulumi Gestalt

## Purpose

This crate is responsible for:
- Establishing and managing gRPC connections to Pulumi services
- Converting between domain-level types and Protocol Buffer (protobuf) types
- Handling asynchronous resource registration and invocation requests
- Managing Pulumi stack lifecycle (root stack creation and tracking)
- Processing secret values and unknown values in a Pulumi-compatible way

## Key Modules

### 1. `output_id` - Output Tracking

- **`OutputId`** - Type-safe wrapper for unique identification of resource outputs
- Implements `Clone`, `Debug`, `Hash`, `PartialEq`, `Eq`, `Ord`, `PartialOrd`
- Used in `PulumiState` to correlate async responses with their originating requests

### 2. `real_pulumi_connector` - Main Connector

**`RealPulumiConnector`** - Implements `PulumiConnector` trait from domain crate

**Methods:**
- `new()` - Creates connector, connects to services, creates root stack
- `register_resource()` - Registers a resource with Pulumi
- `resource_invoke()` - Invokes a resource function
- `register_outputs()` - Registers stack outputs

**Type conversion utilities:**
- `create_protobuf_struct()` - Converts `NodeValue` to protobuf `Struct`
  - Handles `NodeValue::Nothing` by converting to UNKNOWN_VALUE UUID
  - Wraps secret values in special signature container
- `create_map_of_node_values()` - Converts protobuf `Struct` to domain types
  - Detects secret markers in JSON structure
- `json_to_protobuf()` / `protobuf_to_json()` - Recursive value conversion

### 3. `pulumi_state` - Lower-Level gRPC Manager

**`PulumiState`** - Lower-level gRPC state manager for concurrent operations

**Methods:**
- `new()` - Creates state by connecting to monitor and engine
- `send_register_resource_request()` - Queues resource registration
- `send_resource_invoke_request()` - Queues resource invocation
- `register_resource_outputs()` - Synchronously registers stack outputs
- `get_created_resources()` - Non-blocking poll for completed operations

**Internal coordination:**
- Uses `tokio::task::JoinSet` to manage background async tasks
- Responses encoded as protobuf byte vectors and tagged with `OutputId`
- Provides non-blocking retrieval of completed results

### 4. `constants` - Pulumi-Specific Constants

Defines Pulumi-specific constants for secret handling:
- `SPECIAL_SIG_KEY` - Marker key for secret values in JSON
- `SPECIAL_SECRET_SIG` - Marker value confirming something is a secret
- `SECRET_VALUE_NAME` - Field name containing the actual secret value
- `UNKNOWN_VALUE` - UUID for undefined/unknown values

### 5. `test_server` - Mock gRPC Servers

Mock gRPC servers for testing:
- `MyResourceMonitorServer` - Implements `ResourceMonitor` trait
- `MyResourceEngineServer` - Implements `Engine` trait
- Simulates async operation completion timing

## Important Types and Traits

### From This Crate

1. **OutputId** - Type-safe wrapper for output identification
2. **RealPulumiConnector** - Main connector implementation

### From Dependencies

1. **PulumiConnector** trait (from `pulumi_gestalt_domain::connector`)
2. **Tonic gRPC Clients** (from `pulumi_gestalt_proto`)
   - `ResourceMonitorClient<Channel>`
   - `EngineClient<Channel>`
3. **Protobuf Types** (from `pulumi_gestalt_proto::pulumi::pulumirpc`)
   - `RegisterResourceRequest` / `RegisterResourceResponse`
   - `ResourceInvokeRequest` / `InvokeResponse`
   - `RegisterResourceOutputsRequest`

## Testing Approaches

Comprehensive async tests in both `pulumi_state.rs` and `real_pulumi_connector.rs`:

1. **Mock Server Setup** - Spawns local Tonic servers on random ports
2. **Async Concurrency Testing** - Verifies concurrent request handling
3. **Type Conversion Testing** - Validates serialization round-trips

## Dependencies

**Core Dependencies:**
- `tonic` - gRPC framework
- `prost` - Protocol Buffer serialization
- `tokio` - Async runtime and task management
- `futures` - Future utilities
- `anyhow` - Error handling
- `async-trait` - Async trait method support

**Domain Dependencies:**
- `pulumi_gestalt_proto` - Pre-generated protobuf code
- `pulumi_gestalt_domain` - Domain types and connector trait

**Utilities:**
- `serde_json` - JSON value handling
- `prost-types` - Additional protobuf utilities
- `log` - Structured logging

## Special Considerations

### Secret Value Handling
Pulumi marks sensitive values with special JSON structure:
- **Outbound:** Wrap secret values in struct with marker keys
- **Inbound:** Detect marker keys to identify secrets
- **Critical for security** - always preserve the secret flag

### Unknown/Nothing Values
- UUID constant `UNKNOWN_VALUE` represents undefined values
- Must be preserved in round-trip conversions
- Different from null values (which are valid)

### Async Coordination Patterns

**PulumiState (Lower-level):**
- Fire-and-forget request queuing with `JoinSet`
- Manual polling for results
- Non-blocking completion detection

**RealPulumiConnector (Higher-level):**
- Awaitable request methods
- Direct response return
- Automatic type conversion

### gRPC Connection Management
- TCP connections specified as `tcp://host:port` format
- Connections cloned for each request (Tonic handles multiplexing)
- Always initialize with `new()` to ensure proper stack setup

### Root Stack Management
- Every Pulumi state requires a root Stack resource
- Automatically created during `new()`
- Used as parent context for resource registration

### Error Context
Uses `anyhow::Context` for clear error chains:
- Always add context when propagating errors
- Helps with debugging across async boundaries

## Related Crates

- `pulumi_gestalt_domain` - Domain types and connector trait
- `pulumi_gestalt_proto` - Generated protobuf code
