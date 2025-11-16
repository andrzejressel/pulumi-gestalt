# Rust

!!! note "Please Read First"
    Before processing, make sure to learn about [Pulumi](https://www.pulumi.com/tutorials/) and read [Pulumi Gestalt overview](../overview.md)

!!! note "TL/DR"
    Example project is available at [pulumi-gestalt-example](https://github.com/andrzejressel/pulumi-gestalt-example)

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- [Just](https://github.com/casey/just)
- Pulumi Gestalt language plugin: `pulumi plugin install language gestalt "<PULUMI_GESTALT_VERSION>" --server github://api.github.com/andrzejressel/pulumi-gestalt`

## Project setup

### Add dependencies

```toml title="Cargo.toml"
[dependencies]
pulumi_gestalt_rust = { version = "=<PULUMI_GESTALT_VERSION>" }
anyhow = "1.0.95"
bon = "3.3.1"

[build-dependencies]
pulumi_gestalt_build = { version = "=<PULUMI_GESTALT_VERSION>" }
```

### Generate provider code

```rust title="build.rs"
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    pulumi_gestalt_build::generate("random", "4.15.0")?;
    Ok(())
}
```

### Include provider code

```rust title="src/random.rs"
pulumi_gestalt_rust::include_provider!("random");
```

### Use provider

```rust title="src/main.rs"
mod random;
use anyhow::Result;
use random::random_string;
use random::random_string::RandomStringArgs;
use pulumi_gestalt_rust::*;

fn main() {
    run(pulumi_main).unwrap();
}

fn pulumi_main(context: &Context) -> Result<()> {
    let length: Output<i32> = context.new_output(&4);
    let random_string_1 = random_string::create(
        context,
        "test_1",
        RandomStringArgs::builder().length(length).build_struct(),
    );

    let new_length = random_string_1.result.map(|s| s.len() as i32);

    let random_string_2 = random_string::create(
        context,
        "test_2",
        RandomStringArgs::builder()
            .length(new_length)
            .build_struct(),
    );

    let random_string_3 = random_string::create(
        context,
        "test_3",
        RandomStringArgs::builder()
            .length(random_string_2.length.map(|i| i * 2))
            .build_struct(),
    );

    add_export("result", &random_string_1.result);
    add_export("number_1", &random_string_1.length);
    add_export("number_2", &random_string_2.length);
    add_export("number_3", &random_string_3.length);
    Ok(())
}
```

### Add Pulumi.yaml


```yaml title="Pulumi.yaml"
name: Pulumi-Gestalt-Example
runtime: gestalt
```

### Add justfile

```justfile title="justfile" 
run:
    cargo run
```

You can now setup Pulumi stack using `pulumi stack` and run program using `pulumi up`