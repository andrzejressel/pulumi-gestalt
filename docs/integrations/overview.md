# Overview

!!! note "Walk around the code first"
    Before processing, feel free to do [Rust tutorial](../languages/rust/index.md), open code in your IDE, walk through it and check abstractions behind `Output` and generated provider.


## Abstractions

The `Pulumi Gestalt` SDK provides abstractions for working with Pulumi in a structured way. It is built around three core concepts:

1. **Context**: Represents a single execution environment for Pulumi operations. It is not recommended to use a global context, 
   as doing so may limit compatibility with advanced features like the [Pulumi Automation API](https://www.pulumi.com/docs/iac/using-pulumi/automation-api/).
2. **Output**: Represents values that are resolved asynchronously (e.g., cloud resource outputs). Output is usually connected to underlying `value`. Since `Output` can contains
    any type, `value` is JSON representation of it. For example string `hello world` will we `"hello world"`, while number `42` will be `42`. Combining these two values will yield `["hello world", 42]`.
3. **CompositeOutput**: Represents a map of outputs returned by a resource or function. This object is an abstration over map containing resource/function outputs.
   For example [Docker image](https://www.pulumi.com/registry/packages/docker/api-docs/image/) will have 7 - `baseImageName`, `context`, `dockerfile`, `id`, `registryServer`, `repoDigest` and `platform`.

### Context

The `Context` abstraction manages the lifecycle of Pulumi operations. It includes the following methods:

#### Create context

!!! abstract "Initializes a new context. This should be called at the start of your program."

    === "Wasm"

        **🛠️ Signature:**
        ```wit
        package component:pulumi-gestalt@0.0.0-DEV;

        interface context {
            resource context {
                constructor();
            }
        }
        ```

        **📤 Returns:**


        | Type      | Description          |
        |-----------|----------------------|
        | `context` | Instance of context  |

    === "Rust"

        **🛠️ Signature:**
        [docs.rs](https://docs.rs/pulumi_gestalt_rust/latest/pulumi_gestalt_rust/type.Context.html#method.new)
        ```rust
        pub type Context = NativeContext;

        impl NativeContext {
            pub fn new() -> NativeContext {
                NativeContext {
                    inner: integration::Context::create_context(),
                }
            }
        }
        ```

        **📤 Returns:**


        | Type      | Description          |
        |-----------|----------------------|
        | `Context` | Instance of context  |

    === "C FFI"

        **🛠️ Signature:**
        ```c
        typedef struct pulumi_context_t pulumi_context_t;

        struct pulumi_context_t *pulumi_create_context(const void *context);
        ```

        **📥 Parameters:**

        | Name      | Type           | Description                                                 |
        |-----------|----------------|-------------------------------------------------------------|
        | `context` | `const void *` | Pointer that will be passed to `pulumi_output_map` callback |

        **📤 Returns:**

        | Type               | Description          |
        |--------------------|----------------------|
        | `pulumi_context_t` | Instance of context  |


#### `finish`

!!! warning "C FFI and Rust only"
    This function does exist in WIT, but it has [a completely different signature and meaning](wasm.md#callback-emulation).

!!! abstract "Executes all registered operations. Call this before destroying the context."

    === "Wasm"
        This function has [a completely different signature and meaning](wasm.md#callback-emulation).

    === "Rust"
        **🛠️ Signature:**
        ```python
        def finish(ctx: Context)
        ```

        **📥 Parameters:**

        | Name  | Type      | Description         |
        |-------|-----------|---------------------|
        | `ctx` | `Context` | Instance of context |

    === "C FFI"

        **🛠️ Signature:**
        ```c
        void pulumi_finish(struct pulumi_context_t *ctx);
        ```

        **📥 Parameters:**

        | Name  | Type                | Description         |
        |-------|---------------------|---------------------|
        | `ctx` | `pulumi_context_t*` | Instance of context |

#### `destroy_context`

!!! warning "Only in C FFI"
    This function is only available in C FFI. In Rust and WASM, the context is automatically destroyed.

!!! abstract "Cleans up the context. This should be called at the end of your program."

    **🛠️ Signature:**
    ```c
    void pulumi_destroy_context(struct pulumi_context_t *ctx);
    ```

    **📥 Parameters:**

    | Name  | Type                | Description         |
    |-------|---------------------|---------------------|
    | `ctx` | `pulumi_context_t*` | Instance of context |

#### `create_output`

!!! abstract "Creates an `Output` from a known value"

    === "Wasm"

        **🛠️ Signature:**
        ```python
        def create_output(ctx: Context, value: string, secret: bool) -> Output
        ```

        **📥 Parameters:**

        | Name      | Type      | Description         |
        |-----------|-----------|---------------------|
        | `ctx`     | `Context` | Instance of context |
        | `value`   | `string`  | JSON encoded value  |
        | `secret`  | `bool`    | Mark output as secret |

        **📤 Returns:**

        | Type     | Description                                                                                                       |
        |----------|-------------------------------------------------------------------------------------------------------------------|
        | `Output` | An `Output` containing `value`. Does not have to be freed. It will be freed automatically when destroying context |

    === "Rust"

        **🛠️ Signature:**
        ```python
        def create_output(ctx: Context, value: string, secret: bool) -> Output
        ```

        **📥 Parameters:**

        | Name      | Type      | Description         |
        |-----------|-----------|---------------------|
        | `ctx`     | `Context` | Instance of context |
        | `value`   | `string`  | JSON encoded value  |
        | `secret`  | `bool`    | Mark output as secret |

        **📤 Returns:**

        | Type     | Description                                                                                                       |
        |----------|-------------------------------------------------------------------------------------------------------------------|
        | `Output` | An `Output` containing `value`. Does not have to be freed. It will be freed automatically when destroying context |

    === "C FFI"

        **🛠️ Signature:**
        ```c
        struct pulumi_output_t *pulumi_create_output(struct pulumi_context_t *ctx,
                                                     const char *value,
                                                     bool secret);
        ```

        **📥 Parameters:**

        | Name      | Type                | Description         |
        |-----------|---------------------|---------------------|
        | `ctx`     | `pulumi_context_t*` | Instance of context |
        | `value`   | `const char*`       | JSON encoded value  |
        | `secret`  | `bool`              | Mark output as secret |

        **📤 Returns:**

        | Type              | Description                                                                                                       |
        |-------------------|-------------------------------------------------------------------------------------------------------------------|
        | `pulumi_output_t*` | An `Output` containing `value`. Does not have to be freed. It will be freed automatically when destroying context |

#### `register_resource`

!!! abstract "Registers a new resource in the context"

    === "Wasm"

        **🛠️ Signature:**
        ```python
        def register_resource(ctx: Context, type: string, name: string, version: string, inputs: List[ObjectField]) -> CompositeOutput
        ```

        **ObjectField**

        | Name      | Type              | Description         |
        |-----------|-------------------|---------------------|
        | name      | string            | Resource name       |
        | value     | Output            | Resource value      |


        **📥 Parameters:**

        | Name      | Type                | Description                                                  |
        |-----------|---------------------|--------------------------------------------------------------|
        | `ctx`     | `Context`           | Instance of context                                          |
        | `type`    | `string`            | Resource type (i.e `random:index/randomString:RandomString`) |
        | `name`    | `string`            | User's resource name (i.e. `my_resource`)                    |
        | `version` | `string`            | Resource provider version                                    |
        | `inputs`  | `List[ObjectField]` | Resource inputs                                              |

        **📤 Returns:**

        | Type             | Description                                                                                                       |
        |------------------|-------------------------------------------------------------------------------------------------------------------|
        | `CompositeOutput` | An `CompositeOutput` containing resource outputs. Does not have to be freed. It will be freed automatically when destroying context |

    === "Rust"

        **🛠️ Signature:**
        ```python
        def register_resource(ctx: Context, type: string, name: string, version: string, inputs: List[ObjectField]) -> CompositeOutput
        ```

        **ObjectField**

        | Name      | Type              | Description         |
        |-----------|-------------------|---------------------|
        | name      | string            | Resource name       |
        | value     | Output            | Resource value      |


        **📥 Parameters:**

        | Name      | Type                | Description                                                  |
        |-----------|---------------------|--------------------------------------------------------------|
        | `ctx`     | `Context`           | Instance of context                                          |
        | `type`    | `string`            | Resource type (i.e `random:index/randomString:RandomString`) |
        | `name`    | `string`            | User's resource name (i.e. `my_resource`)                    |
        | `version` | `string`            | Resource provider version                                    |
        | `inputs`  | `List[ObjectField]` | Resource inputs                                              |

        **📤 Returns:**

        | Type             | Description                                                                                                       |
        |------------------|-------------------------------------------------------------------------------------------------------------------|
        | `CompositeOutput` | An `CompositeOutput` containing resource outputs. Does not have to be freed. It will be freed automatically when destroying context |

    === "C FFI"

        **🛠️ Signature:**
        ```c
        typedef struct pulumi_object_field_t {
          const char *name;
          const struct pulumi_output_t *value;
        } pulumi_object_field_t;

        typedef struct pulumi_register_resource_request_t {
          const char *type_;
          const char *name;
          const char *version;
          const struct pulumi_object_field_t *inputs;
          uintptr_t inputs_len;
        } pulumi_register_resource_request_t;

        struct pulumi_composite_output_t *pulumi_register_resource(struct pulumi_context_t *ctx,
                                                                   const struct pulumi_register_resource_request_t *request);
        ```

        **📥 Parameters:**

        | Name      | Type                                | Description                                                  |
        |-----------|-------------------------------------|--------------------------------------------------------------|
        | `ctx`     | `pulumi_context_t*`                 | Instance of context                                          |
        | `request` | `pulumi_register_resource_request_t*` | Request containing type, name, version, and inputs           |

        **📤 Returns:**

        | Type                       | Description                                                                                                       |
        |----------------------------|-------------------------------------------------------------------------------------------------------------------|
        | `pulumi_composite_output_t*` | An `CompositeOutput` containing resource outputs. Does not have to be freed. It will be freed automatically when destroying context |

#### `invoke_resource`

!!! abstract "Invokes a resource (also referred to as a `function`)"

    === "Wasm"

        **🛠️ Signature:**
        ```python
        def invoke_resource(ctx: Context, token: string, version: string, inputs: List[ObjectField]) -> CompositeOutput
        ```

        **ObjectField**

        | Name      | Type              | Description         |
        |-----------|-------------------|---------------------|
        | name      | string            | Resource name       |
        | value     | Output            | Resource value      |


        **📥 Parameters:**

        | Name      | Type                | Description                                                       |
        |-----------|---------------------|-------------------------------------------------------------------|
        | `ctx`     | `Context`           | Instance of context                                               |
        | `token`   | `string`            | Resource token (i.e [`docker:index/getNetwork:getNetwork`](https://github.com/pulumi/pulumi-docker/blob/v4.6.1/provider/cmd/pulumi-resource-docker/schema.json#L4395)) |
        | `version` | `string`            | Resource provider version                                         |
        | `inputs`  | `List[ObjectField]` | Resource inputs                                                   |


        **📤 Returns:**

        | Type             | Description                                                                                                       |
        |------------------|-------------------------------------------------------------------------------------------------------------------|
        | `CompositeOutput` | An `CompositeOutput` containing resource outputs. Does not have to be freed. It will be freed automatically when destroying context |

    === "Rust"

        **🛠️ Signature:**
        ```python
        def invoke_resource(ctx: Context, token: string, version: string, inputs: List[ObjectField]) -> CompositeOutput
        ```

        **ObjectField**

        | Name      | Type              | Description         |
        |-----------|-------------------|---------------------|
        | name      | string            | Resource name       |
        | value     | Output            | Resource value      |


        **📥 Parameters:**

        | Name      | Type                | Description                                                       |
        |-----------|---------------------|-------------------------------------------------------------------|
        | `ctx`     | `Context`           | Instance of context                                               |
        | `token`   | `string`            | Resource token (i.e [`docker:index/getNetwork:getNetwork`](https://github.com/pulumi/pulumi-docker/blob/v4.6.1/provider/cmd/pulumi-resource-docker/schema.json#L4395)) |
        | `version` | `string`            | Resource provider version                                         |
        | `inputs`  | `List[ObjectField]` | Resource inputs                                                   |


        **📤 Returns:**

        | Type             | Description                                                                                                       |
        |------------------|-------------------------------------------------------------------------------------------------------------------|
        | `CompositeOutput` | An `CompositeOutput` containing resource outputs. Does not have to be freed. It will be freed automatically when destroying context |

    === "C FFI"

        **🛠️ Signature:**
        ```c
        typedef struct pulumi_invoke_resource_request_t {
          const char *token;
          const char *version;
          const struct pulumi_object_field_t *inputs;
          uintptr_t inputs_len;
        } pulumi_invoke_resource_request_t;

        struct pulumi_composite_output_t *pulumi_invoke_resource(struct pulumi_context_t *ctx,
                                                                 const struct pulumi_invoke_resource_request_t *request);
        ```

        **📥 Parameters:**

        | Name      | Type                                | Description                                                       |
        |-----------|-------------------------------------|-------------------------------------------------------------------|
        | `ctx`     | `pulumi_context_t*`                 | Instance of context                                               |
        | `request` | `pulumi_invoke_resource_request_t*` | Request containing token, version, and inputs                     |


        **📤 Returns:**

        | Type                       | Description                                                                                                       |
        |----------------------------|-------------------------------------------------------------------------------------------------------------------|
        | `pulumi_composite_output_t*` | An `CompositeOutput` containing resource outputs. Does not have to be freed. It will be freed automatically when destroying context |

#### `get_config`

!!! abstract "Receives value from [config](https://www.pulumi.com/docs/iac/concepts/config/)"

    === "Wasm"

        **🛠️ Signature:**
        ```python
        def get_config(ctx: Context, name: Option[String], key: String) -> Option[ConfigValue]
        ```

        **ConfigValue**

        | Name      | Type              | Description         |
        |-----------|-------------------|---------------------|
        | plaintext | string            | Config value if it is not secret       |
        | secret    | Output            | Config value hidden in output if it is a secret      |


        **📥 Parameters:**

        | Name      | Type              | Description                                                       |
        |-----------|-------------------|-------------------------------------------------------------------|
        | `ctx`     | `Context`         | Instance of context                                               |
        | `name`    | `Option[string]`  | Config namespace |
        | `key`     | `string`          | Config key                                       |


        **📤 Returns:**

        | Type             | Description                                                                                                       |
        |------------------|-------------------------------------------------------------------------------------------------------------------|
        | `Option[ConfigValue]` | None if config does not exist, String if it is plaintext, String hidden in output if it is secret |

    === "Rust"

        **🛠️ Signature:**
        ```python
        def get_config(ctx: Context, name: Option[String], key: String) -> Option[ConfigValue]
        ```

        **ConfigValue**

        | Name      | Type              | Description         |
        |-----------|-------------------|---------------------|
        | plaintext | string            | Config value if it is not secret       |
        | secret    | Output            | Config value hidden in output if it is a secret      |


        **📥 Parameters:**

        | Name      | Type              | Description                                                       |
        |-----------|-------------------|-------------------------------------------------------------------|
        | `ctx`     | `Context`         | Instance of context                                               |
        | `name`    | `Option[string]`  | Config namespace |
        | `key`     | `string`          | Config key                                       |


        **📤 Returns:**

        | Type             | Description                                                                                                       |
        |------------------|-------------------------------------------------------------------------------------------------------------------|
        | `Option[ConfigValue]` | None if config does not exist, String if it is plaintext, String hidden in output if it is secret |

    === "C FFI"

        **🛠️ Signature:**
        ```c
        typedef enum pulumi_config_value_t_Tag {
          PlainValue,
          Secret,
        } pulumi_config_value_t_Tag;

        typedef struct pulumi_config_value_t {
          pulumi_config_value_t_Tag tag;
          union {
            struct {
              char *plain_value;
            };
            struct {
              struct pulumi_output_t *secret;
            };
          };
        } pulumi_config_value_t;

        struct pulumi_config_value_t *pulumi_config_get_value(struct pulumi_context_t *ctx,
                                                              const char *name,
                                                              const char *key);

        void pulumi_config_free(struct pulumi_config_value_t *value);
        ```

        **📥 Parameters:**

        | Name      | Type              | Description                                                       |
        |-----------|-------------------|-------------------------------------------------------------------|
        | `ctx`     | `pulumi_context_t*` | Instance of context                                             |
        | `name`    | `const char*`     | Config namespace. If null, the default (project name) is used     |
        | `key`     | `const char*`     | Config key. Cannot be null                                        |


        **📤 Returns:**

        | Type                    | Description                                                                                                       |
        |-------------------------|-------------------------------------------------------------------------------------------------------------------|
        | `pulumi_config_value_t*` | Config value or NULL if not found. Must be freed with `pulumi_config_free` when no longer needed |


### Output

#### map

!!! abstract "Applies a function to transform the value inside an `Output`"

    === "Wasm"

        **🛠️ Signature:**
        ```python
        def map(output: Output, func: Union[A => B, String => String, String]) -> Output;
        ```

        **📥 Parameters:**

        | Name     | Type                                                        | Description                            |
        |----------|-------------------------------------------------------------|----------------------------------------|
        | `output` | `Output`                                                    | An `Output` object to transform        |
        | `func`   | One of:<br />`A => B`<br />`string => string`<br />`string` | Function to apply to the `Output`      |

        **📤 Returns:**

        | Type     | Description                                                                                                       |
        |----------|-------------------------------------------------------------------------------------------------------------------|
        | `Output` | An `Output` containing transformed value. Does not have to be freed. It will be freed automatically when destroying context |

    === "Rust"

        **🛠️ Signature:**
        ```python
        def map(output: Output, func: Union[A => B, String => String, String]) -> Output;
        ```

        **📥 Parameters:**

        | Name     | Type                                                        | Description                            |
        |----------|-------------------------------------------------------------|----------------------------------------|
        | `output` | `Output`                                                    | An `Output` object to transform        |
        | `func`   | One of:<br />`A => B`<br />`string => string`<br />`string` | Function to apply to the `Output`      |

        **📤 Returns:**

        | Type     | Description                                                                                                       |
        |----------|-------------------------------------------------------------------------------------------------------------------|
        | `Output` | An `Output` containing transformed value. Does not have to be freed. It will be freed automatically when destroying context |

    === "C FFI"

        **🛠️ Signature:**
        ```c
        /**
         * Arguments: Engine context, Function context, Serialized JSON value
         * Returned string must represent a JSON value;
         * Library will free the returned string
         */
        typedef char *(*pulumi_mapping_function_t)(const void*, const void*, const char*);

        struct pulumi_output_t *pulumi_output_map(struct pulumi_context_t *ctx,
                                                  const struct pulumi_output_t *output,
                                                  const void *function_context,
                                                  pulumi_mapping_function_t function);
        ```

        **📥 Parameters:**

        | Name               | Type                      | Description                                                  |
        |--------------------|---------------------------|--------------------------------------------------------------|
        | `ctx`              | `pulumi_context_t*`       | Instance of context                                          |
        | `output`           | `const pulumi_output_t*`  | An `Output` object to transform                              |
        | `function_context` | `const void*`             | Context that will be passed to the mapping function callback |
        | `function`         | `pulumi_mapping_function_t` | Function to apply to the `Output`                          |

        **📤 Returns:**

        | Type               | Description                                                                                                       |
        |--------------------|-------------------------------------------------------------------------------------------------------------------|
        | `pulumi_output_t*` | An `Output` containing transformed value. Does not have to be freed. It will be freed automatically when destroying context |

#### combine

!!! abstract "Combines multiple `Output` objects to create a single composite `Output`"

    === "Wasm"

        **🛠️ Signature:**
        ```python
        def combine(output: Output, outputs: List[Output]) -> Output;
        ```

        **📥 Parameters:**

        | Name      | Type           | Description                         |
        |-----------|----------------|-------------------------------------|
        | `output`  | `Output`       | `this` output                       |
        | `outputs` | `List[Output]` | List of `Output` objects to combine |

        **📤 Returns:**

        | Type     | Description                                                                                                       |
        |----------|-------------------------------------------------------------------------------------------------------------------|
        | `Output` | An `Output` containing combined values. The structure looks like `[output, outputs[0], outputs[1], ...]` Does not have to be freed. It will be freed automatically when destroying context |

    === "Rust"

        **🛠️ Signature:**
        ```python
        def combine(output: Output, outputs: List[Output]) -> Output;
        ```

        **📥 Parameters:**

        | Name      | Type           | Description                         |
        |-----------|----------------|-------------------------------------|
        | `output`  | `Output`       | `this` output                       |
        | `outputs` | `List[Output]` | List of `Output` objects to combine |

        **📤 Returns:**

        | Type     | Description                                                                                                       |
        |----------|-------------------------------------------------------------------------------------------------------------------|
        | `Output` | An `Output` containing combined values. The structure looks like `[output, outputs[0], outputs[1], ...]` Does not have to be freed. It will be freed automatically when destroying context |

    === "C FFI"

        **🛠️ Signature:**
        ```c
        struct pulumi_output_t *pulumi_output_combine(const struct pulumi_output_t *output,
                                                      const struct pulumi_output_t *const *outputs,
                                                      uintptr_t outputs_size);
        ```

        **📥 Parameters:**

        | Name           | Type                          | Description                         |
        |----------------|-------------------------------|-------------------------------------|
        | `output`       | `const pulumi_output_t*`      | `this` output                       |
        | `outputs`      | `const pulumi_output_t**`     | Array of `Output` objects to combine |
        | `outputs_size` | `uintptr_t`                   | Size of the outputs array           |

        **📤 Returns:**

        | Type               | Description                                                                                                       |
        |--------------------|-------------------------------------------------------------------------------------------------------------------|
        | `pulumi_output_t*` | An `Output` containing combined values. The structure looks like `[output, outputs[0], outputs[1], ...]` Does not have to be freed. It will be freed automatically when destroying context |


#### add_to_export

!!! abstract "Add output as [stack output](https://www.pulumi.com/tutorials/building-with-pulumi/stack-outputs/)."

    === "Wasm"

        **🛠️ Signature:**
        ```python
        def add_to_export(output: Output, name: string);
        ```

        **📥 Parameters:**

        | Name     | Type     | Description                            |
        |----------|----------|----------------------------------------|
        | `output` | `Output` | `Output` object to add as stack output |
        | `name`   | `string` | Name of the stack output               |

    === "Rust"

        **🛠️ Signature:**
        ```python
        def add_to_export(output: Output, name: string);
        ```

        **📥 Parameters:**

        | Name     | Type     | Description                            |
        |----------|----------|----------------------------------------|
        | `output` | `Output` | `Output` object to add as stack output |
        | `name`   | `string` | Name of the stack output               |

    === "C FFI"

        **🛠️ Signature:**
        ```c
        void pulumi_output_add_to_export(const struct pulumi_output_t *value, const char *name);
        ```

        **📥 Parameters:**

        | Name    | Type                     | Description                            |
        |---------|--------------------------|----------------------------------------|
        | `value` | `const pulumi_output_t*` | `Output` object to add as stack output |
        | `name`  | `const char*`            | Name of the stack output               |


### CompositeOutput

#### get_field

!!! abstract "Get resource operation result value"

    === "Wasm"

        **🛠️ Signature:**
        ```python
        def get_field(output: CompositeOutput, field: string) -> Output;
        ```

        **📥 Parameters:**

        | Name     | Type              | Description                            |
        |----------|-------------------|----------------------------------------|
        | `output` | `CompositeOutput` | `CompositeOutput` object to get field  |
        | `field`  | `string`          | Field name                             |

        **📤 Returns:**

        | Type     | Description                                                                                                       |
        |----------|-------------------------------------------------------------------------------------------------------------------|
        | `Output` | An `Output` containing field value. Does not have to be freed. It will be freed automatically when destroying context |

    === "Rust"

        **🛠️ Signature:**
        ```python
        def get_field(output: CompositeOutput, field: string) -> Output;
        ```

        **📥 Parameters:**

        | Name     | Type              | Description                            |
        |----------|-------------------|----------------------------------------|
        | `output` | `CompositeOutput` | `CompositeOutput` object to get field  |
        | `field`  | `string`          | Field name                             |

        **📤 Returns:**

        | Type     | Description                                                                                                       |
        |----------|-------------------------------------------------------------------------------------------------------------------|
        | `Output` | An `Output` containing field value. Does not have to be freed. It will be freed automatically when destroying context |

    === "C FFI"

        **🛠️ Signature:**
        ```c
        struct pulumi_output_t *pulumi_composite_output_get_field(struct pulumi_composite_output_t *output,
                                                                  const char *field_name);
        ```

        **📥 Parameters:**

        | Name         | Type                        | Description                            |
        |--------------|----------------------------|----------------------------------------|
        | `output`     | `pulumi_composite_output_t*` | `CompositeOutput` object to get field  |
        | `field_name` | `const char*`               | Field name                             |

        **📤 Returns:**

        | Type               | Description                                                                                                       |
        |--------------------|-------------------------------------------------------------------------------------------------------------------|
        | `pulumi_output_t*` | An `Output` containing field value. Does not have to be freed. It will be freed automatically when destroying context |

### Abstraction Levels for `Output::map`

The `Output::map` function is implemented at different levels of abstraction depending on the language or integration:

#### High Level
- **Description**: Pass a function of type `T1 -> T2` to `Output::map`.
- **Examples**: [Rust language](../languages/rust/index.md).

#### Medium Level
- **Description**: Pass a function of type `String -> String` to `Output::map` where `String` is a serialized JSON value.
- **Examples**: [C FFI](c-ffi.md), [Rust integration](rust.md).

#### Low Level
- **Description**: Pass a function id to `Output::map` and receive it later.
- **Examples**: [Wasm](wasm.md).
