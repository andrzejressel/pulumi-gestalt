# Overview

## Pulumi Gestalt Rust Language plugin

Pulumi Gestalt uses a custom language plugin that can be installed using Pulumi CLI:

```shell
pulumi plugin install language rust "VERSION" --server github://api.github.com/andrzejressel/pulumi-gestalt
```

Simple [project file](https://www.pulumi.com/docs/iac/concepts/projects/) looks like the following

```yaml title="Pulumi.yaml"
name: Some_name
runtime: rust
```

In this case plugin will use `cargo build`  to build and `cargo run` to run the program.

There is single option for rust: `binary` - it is location of binary that will be run instead of `cargo run`
```yaml title="Pulumi.yaml"
name: Some_name
options:
  name: rust
  binary: "path/to/binary"
```
