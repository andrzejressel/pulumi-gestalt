---
description: Modifies the PCL -> Rust generator to support new expression type or new syntax.
---

Project structure:

- @proto/language_server.proto - Protobuf definition of pcl program structure
- @pulumi-language-rust/ast/protobuf/schemapcl/pcl.pb.go - File generate from above file
- @pulumi-language-rust/ast/codegen/ast/program.go - Mapper of PCL into Protobuf
- @crates/rust_language_server/src - Rust generator that uses the protobuf definitions to generate Rust code.


Useful Just recipies:
- `regenerate` in @pulumi-language-rust/justfile - Regenerates go and rust code for protobuf definitions.


Your task is:
<task>
$ARGUMENTS
</task>