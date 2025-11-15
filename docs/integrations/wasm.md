# Wasm

!!! note "Please Read First"
    Before proceeding, make sure to read [Pulumi Gestalt integrations Overview](overview.md) page to get a better
    understanding of the documentation.

In Pulumi Gestalt, Wasm support is based on the [Component Model](https://component-model.bytecodealliance.org/).

## Artifacts

Artifacts related to Wasm support can be found on
the [releases](https://github.com/andrzejressel/pulumi-gestalt/releases/) page:

- **Runner** (`pulumi_gestalt_wasm_runner`)
- **WIT files** (`world.wit`)

## Versioning

Runner and WIT files are versioned after the whole project without any 
forward or backward compatibility. This means that if you want to use a specific version of the Runner, you need to use WIT
files from the same version.

## Entrypoint

The entry point is managed by the `component:pulumi-gestal/pulumi-main#pulumi-main` interface. This function is invoked by
`pulumi-gestalt-runner`.

## Callback Emulation

In Pulumi, one of the key features is the ability to invoke arbitrary code for Output mapping. Typically, an Output
would be implemented as a `Future<T>` abstraction with a `map` method. However, this introduces a circular dependency:
user code needs `Future<T>.map`, but `Future<T>.map` needs user code. While this is not a problem in most languages, it
is an issue in the WebAssembly (Wasm) Component Model due to its lack of support for circular dependencies.

To address this, Pulumi Gestalt uses a callback emulation mechanism where user code must manually retrieve a list of
required callbacks, invoke them, and return the results. This is done using the `finish` function, which has the
following signature:

```wit
interface types {
    record function-invocation-request {
        id: output,
        function-name: string,
        value: string,
    }
    record function-invocation-result {
        id: borrow<output>,
        value: string,
    }
}
interface output-interface {
    resource output {
        map: func(function-name: string) -> output;
    }
}
interface context {
    resource context {
        finish: func(functions: list<function-invocation-result>) -> list<function-invocation-request>;
    }
}
```

The algorithm is as follows:

```mermaid
sequenceDiagram
    User -> User: Assign name to function
    User -> User: Add name and function to global map
    User -> Pulumi_Gestalt_Component: Map given output with function name
    User -> User: Run rest of the program
    User -> User: Assign empty list to `results`
    loop forever
        User -> Pulumi_Gestalt_Component: Invoke `finish` with `results`
        Note right of Pulumi_Gestalt_Component: Returns list of `function-invocation-request`
        User -> User: If list is empty, exit loop
        loop Compute results
            User -> User: For each request in list
            User -> User: Look up function by `function-id`
            User -> User: Invoke function with `value`
            User -> User: Create `function-invocation-result`
            User -> User: Add result to `results`
        end
    end
```

You can check implementation of this mechanism inside `Pulumi Gestalt` itself:
- [First implementation](https://docs.rs/pulumi_gestalt_rust_integration/latest/pulumi_gestalt_rust_integration/struct.Context.html#method.finish)
- [Second implementation (`run_loop` function)](https://docs.rs/pulumi_gestalt_rust_adapter_wasm/latest/src/pulumi_gestalt_rust_adapter_wasm/runner.rs.html)

## Runner Quick Start

To execute a Pulumi Gestalt Wasm program using the Runner, use:

```sh
pulumi_wasm_runner run <WASM_FILE>
```

## WIT Files

```title="world.wit"
--8<-- "crates/wit/wit/world.wit"
```

## Example

An example of Wasm integration in Rust

```rust title="lib.rs"
--8<-- "examples/wasm/src/lib.rs"
```