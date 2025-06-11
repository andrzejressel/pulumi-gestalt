# Pulumi Gestalt Protobuf Schema

This document provides a high-level overview of the `pulumi_gestalt.proto` schema, which defines the structure for representing Pulumi package schemas in Protobuf format. Detailed descriptions of individual fields and messages can be found in the comments directly within the `pulumi_gestalt.proto` file. The purpose of this document is to explain the overall structure and the relationships between key components.

## Core Concepts

The schema is designed to mirror the information typically found in a Pulumi `schema.json` file, but in a structured and strongly-typed protobuf format. The main components are:

### 1. `Package`
The main message, `Package`, represents an entire Pulumi package. It contains:
- Basic package metadata (name, version, display name, plugin download URL).
- Collections of resources, functions, and global type definitions provided by the package.

### 2. `Resource`
The `Resource` message describes a manageable cloud or infrastructure component provided by the package (e.g., an AWS S3 Bucket, an Azure Resource Group). Each resource definition includes:
- An `ElementId` for unique identification.
- Input properties required to create or update the resource.
- Output properties that become available after the resource is provisioned.

### 3. `Function`
The `Function` message describes a data source or a callable operation provided by the package (e.g., a function to look up an existing AWS AMI). Similar to resources, functions have:
- An `ElementId`.
- Input parameters.
- Output values.

### 4. Type System (`Type`, `GlobalType`, `RefType`)

The schema defines a rich type system for describing the data types of properties and parameters.

- **`Type`**: This is a versatile message used to define the type of an input/output property or parameter. It can represent:
    - Primitive types (boolean, integer, number, string).
    - Collection types (array, object/map).
    - References to other defined types (`RefType`).
    - Optional types.
    - Discriminated unions (allowing a value to be one of several specified types).
    - Constant string values.

- **`GlobalType`**: This message is used to define complex, reusable types at the package level, such as custom objects or enums. These global types can then be referenced by `Type` messages using `RefType`.
    - **Object Types**: Custom structures with a defined set of properties.
    - **Enum Types**: Enumerations with string, number, or integer values.

- **`RefType`**: Used within the `Type` message to point to:
    - A `GlobalType` defined elsewhere in the package.
    - Special Pulumi-specific types like `Archive`, `Asset`, or `Any`.

### 5. `ElementId`
The `ElementId` message is crucial for uniquely identifying resources, functions, and global types within and across packages. It typically consists of:
- `namespace`: A `repeated string` representing the module path components within the package (e.g., for a token `pkg:moduleA/moduleB:ResourceName`, the namespace could be `["moduleA", "moduleB"]`). The raw token string provides the full context.
- `name`: The name of the element (e.g., `"Bucket"`).
- `raw`: The complete raw token string (e.g., `"aws:s3/bucket:Bucket"`).

This structured approach allows for clear definitions and easier processing of Pulumi schemas compared to raw JSON, especially in multi-language environments or when generating code from schemas.

