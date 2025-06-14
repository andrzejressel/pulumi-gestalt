# Overview

!!! note "Walk around the code first"
    Before processing, feel free to do [Rust tutorial](../languages/rust/index.md), open code in your IDE, walk through it
    and check abstractions behind `Output` and generated provider.


## Schema

The schema for the Pulumi Gestalt Protobuf is defined in the `pulumi_gestalt.proto` file. It includes definitions
for the main components such as `Package`, `Resource`, `Function`, and various types used within these components. More
information about the schema can be found in
the [Gestalt Protocol Buffer Schema documentation](schema.md).

## Abstractions

The `Pulumi Gestalt` SDK provides abstractions for working with Pulumi in a structured way. It is built around three
core concepts:

1. **Context**: Represents a single execution environment for Pulumi operations. It is not recommended to use a global
   context,
   as doing so may limit compatibility with advanced features like
   the [Pulumi Automation API](https://www.pulumi.com/docs/iac/using-pulumi/automation-api/).
2. **Output**: Represents values that are resolved asynchronously (e.g., cloud resource outputs). Output is usually
   connected to underlying `value`. Since `Output` can contain
   any type, `value` is JSON representation of it. For example string `hello world` will be `"hello world"`, while
   number `42` will be `42`. Combining these two values will yield `["hello world", 42]`.
3. **CompositeOutput**: Represents a map of outputs returned by a resource or function. This object is an abstraction
   over map containing resource/function outputs.
   For example [Docker image](https://www.pulumi.com/registry/packages/docker/api-docs/image/) will have 7 -
   `baseImageName`, `context`, `dockerfile`, `id`, `registryServer`, `repoDigest` and `platform`.

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
        [docs.rs](https://docs.rs/pulumi_gestalt_rust_integration/latest/pulumi_gestalt_rust_integration/struct.Context.html#method.create_context)
        ```rust
        pub struct Context { /* private fields */ }

        impl Context {
            pub fn create_context() -> Context { }
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

#### Finish

!!! warning "C FFI and Rust only"
This function does exist in WIT, but it has [a completely different signature and meaning](wasm.md#callback-emulation).

!!! abstract "Executes all registered operations. Call this before destroying the context."

    === "Wasm"

        **🛠️ Signature:**
        ```wit
        package component:pulumi-gestalt@0.0.0-DEV;

        interface output-interface {
            resource output {
            }
        }

        interface types {
            use output-interface.{output};
        
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
        interface context {
            resource context {
                finish: func(functions: list<function-invocation-result>) -> list<function-invocation-request>;
            }
        }
        ```

        **function-invocation-result**

        | Name      | Type              | Description         |
        |-----------|-------------------|---------------------|
        | id        | borrow<output>    | Output ID           |
        | value     | string            | JSON encoded value  |

        **function-invocation-request**

        | Name           | Type              | Description         |
        |----------------|-------------------|---------------------|
        | id             | output            | Output ID           |
        | function-name  | string            | Function name       |
        | value          | string            | JSON encoded value  |

        **📥 Parameters:**

        | Name       | Type                              | Description                     |
        |------------|-----------------------------------|---------------------------------|
        | `functions`| `list<function-invocation-result>` | Results of function invocations |

        **📤 Returns:**

        | Type                              | Description                        |
        |-----------------------------------|------------------------------------|
        | `list<function-invocation-request>` | List of function invocation requests |

        See [callback emulation](wasm.md#callback-emulation) for more details on how this function is used.

    === "Rust"

        **🛠️ Signature:**
        [docs.rs](https://docs.rs/pulumi_gestalt_rust_integration/latest/pulumi_gestalt_rust_integration/struct.Context.html#method.finish)
        ```rust
        pub struct Context { /* private fields */ }

        impl Context {
            pub fn finish(&self)
        }
        ```

        **📥 Parameters:**

        | Name   | Type       | Description         |
        |--------|------------|---------------------|
        | `self` | `&Context` | Instance of context |

    === "C FFI"

        **🛠️ Signature:**
        ```c
        typedef struct pulumi_context_t pulumi_context_t;

        void pulumi_finish(struct pulumi_context_t *ctx);
        ```

        **📥 Parameters:**

        | Name  | Type                | Description         |
        |-------|---------------------|---------------------|
        | `ctx` | `pulumi_context_t*` | Instance of context |

#### Destroy context

!!! abstract "Cleans up the context. This should be called at the end of your program."

    === "Wasm"
    
        _Not applicable. In the Wasm context, the context is destroyed automatically._
    
    === "Rust"
    
        _Not applicable. In the Rust context, the context is destroyed automatically._
    
    === "C FFI"

        **🛠️ Signature:**
        ```c
        typedef struct pulumi_context_t pulumi_context_t;

        void pulumi_destroy_context(struct pulumi_context_t *ctx);
        ```

        **📥 Parameters:**

        | Name  | Type                | Description         |
        |-------|---------------------|---------------------|
        | `ctx` | `pulumi_context_t*` | Instance of context |

#### Create output

!!! abstract "Creates an `Output` from a known value"

    === "Wasm"

        **🛠️ Signature:**
        ```wit
        package component:pulumi-gestalt@0.0.0-DEV;

        interface context {
            resource context {
                create-output: func(value: string, secret:bool) -> output;
            }
        }
        ```

        **📥 Parameters:**

        | Name     | Type     | Description          |
        |----------|----------|----------------------|
        | `value`  | `string` | JSON encoded value   |
        | `secret` | `bool`   | Mark output as secret |

        **📤 Returns:**

        | Type     | Description                    |
        |----------|--------------------------------|
        | `Output` | An `Output` containing `value` |

    === "Rust"

        **🛠️ Signature:**
        [docs.rs](https://docs.rs/pulumi_gestalt_rust_integration/latest/pulumi_gestalt_rust_integration/struct.Context.html#method.create_output)
        ```rust
        pub struct Context { /* private fields */ }

        impl Context {
            pub fn create_output(&self, value: String, secret: bool) -> Output
        }
        ```

        **📥 Parameters:**

        | Name      | Type            | Description         |
        |-----------|-----------------|---------------------|
        | `self`    | `&Context`      | Instance of context |
        | `value`  | `String` | JSON encoded value   |
        | `secret` | `bool`   | Mark output as secret |

        **📤 Returns:**

        | Type        | Description                                                |
        |-------------|------------------------------------------------------------|
        | `Output<T>` | An `Output` containing `value`                             |

    === "C FFI"

        **🛠️ Signature:**
        ```c
        typedef struct pulumi_context_t pulumi_context_t;

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

#### Register resource

!!! abstract "Registers a new resource in the context"

    === "Wasm"

        **🛠️ Signature:**
        ```wit
        package component:pulumi-gestalt@0.0.0-DEV;

        interface context {
            resource context {
                register-resource: func(request: register-resource-request) -> composite-output;
            }
        }
        ```

        **register-resource-request**

        | Name      | Type                | Description                                                  |
        |-----------|---------------------|--------------------------------------------------------------|
        | `%type`   | `string`            | Resource type (i.e `random:index/randomString:RandomString`) |
        | `name`    | `string`            | User's resource name (i.e. `my_resource`)                    |
        | `version` | `string`            | Resource provider version                                    |
        | `object`  | `list<object-field>` | Resource inputs                                             |

        **object-field**

        | Name      | Type              | Description         |
        |-----------|-------------------|---------------------|
        | name      | string            | Resource name       |
        | value     | borrow<output>    | Resource value      |


        **📥 Parameters:**

        | Name      | Type                       | Description                                                  |
        |-----------|----------------------------|--------------------------------------------------------------|
        | `request` | `register-resource-request` | Request containing type, name, version, and inputs           |

        **📤 Returns:**

        | Type             | Description                                                                                                       |
        |------------------|-------------------------------------------------------------------------------------------------------------------|
        | `CompositeOutput` | An `CompositeOutput` containing resource outputs. Does not have to be freed. It will be freed automatically when destroying context |

    === "Rust"

        **🛠️ Signature:**
        [docs.rs](https://docs.rs/pulumi_gestalt_rust_integration/latest/pulumi_gestalt_rust_integration/struct.Context.html#method.register_resource)
        ```rust
        pub struct ObjectField<'a> {
            pub name: String,
            pub value: &'a Output,
        }

        pub struct RegisterResourceRequest<'a> {
            pub type_: String,
            pub name: String,
            pub version: String,
            pub inputs: &'a [ObjectField<'a>],
        }

        pub struct Context { /* private fields */ }

        impl Context {
            pub fn register_resource(
                &self,
                request: RegisterResourceRequest<'_>,
            ) -> CompositeOutput
        }
        ```

        **📥 Parameters:**

        | Name      | Type                          | Description                                                  |
        |-----------|-------------------------------|--------------------------------------------------------------|
        | `self`    | `&Context`                    | Instance of context                                          |
        | `request` | `RegisterResourceRequest<T>`  | Request containing type, name, version, and inputs           |

        **📤 Returns:**

        | Type              | Description                                                                |
        |-------------------|----------------------------------------------------------------------------|
        | `CompositeOutput` | A `CompositeOutput` containing resource outputs                            |

    === "C FFI"

        **🛠️ Signature:**
        ```c
        typedef struct pulumi_context_t pulumi_context_t;
        typedef struct pulumi_output_t pulumi_output_t;

        typedef struct pulumi_object_field_t {
            const char *name;
            const struct pulumi_output_t *value;
        } pulumi_object_field_t;

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

#### Invoke resource

!!! abstract "Invokes a resource (also referred to as a `function`)"

    === "Wasm"

        **🛠️ Signature:**
        ```wit
        package component:pulumi-gestalt@0.0.0-DEV;

        interface context {
            resource context {
                invoke-resource: func(request: resource-invoke-request) -> composite-output;
            }
        }
        ```

        **resource-invoke-request**

        | Name      | Type                | Description                                                       |
        |-----------|---------------------|-------------------------------------------------------------------|
        | `token`   | `string`            | Resource token (i.e [`docker:index/getNetwork:getNetwork`](https://github.com/pulumi/pulumi-docker/blob/v4.6.1/provider/cmd/pulumi-resource-docker/schema.json#L4395)) |
        | `version` | `string`            | Resource provider version                                         |
        | `object`  | `list<object-field>` | Resource inputs                                                  |

        **object-field**

        | Name      | Type              | Description         |
        |-----------|-------------------|---------------------|
        | name      | string            | Resource name       |
        | value     | borrow<output>    | Resource value      |


        **📥 Parameters:**

        | Name      | Type                      | Description                                                       |
        |-----------|---------------------------|-------------------------------------------------------------------|
        | `request` | `resource-invoke-request` | Request containing token, version, and inputs                     |


        **📤 Returns:**

        | Type             | Description                                                                                                       |
        |------------------|-------------------------------------------------------------------------------------------------------------------|
        | `CompositeOutput` | An `CompositeOutput` containing resource outputs. Does not have to be freed. It will be freed automatically when destroying context |

    === "Rust"

        **🛠️ Signature:**
        [docs.rs](https://docs.rs/pulumi_gestalt_rust_integration/latest/pulumi_gestalt_rust_integration/struct.Context.html#method.invoke_resource)
        ```rust
        pub struct ObjectField<'a> {
            pub name: String,
            pub value: &'a Output,
        }

        pub struct InvokeResourceRequest<'a> {
            pub token: String,
            pub version: String,
            pub inputs: &'a [ObjectField<'a>],
        }

        pub struct Context { /* private fields */ }

        impl Context {
            pub fn invoke_resource(
                &self,
                request: InvokeResourceRequest<'_>,
            ) -> CompositeOutput
        }
        ```

        **📥 Parameters:**

        | Name      | Type                        | Description                                                       |
        |-----------|----------------------------|-------------------------------------------------------------------|
        | `self`    | `&Context`                 | Instance of context                                               |
        | `request` | `InvokeResourceRequest<T>` | Request containing token, version, and inputs                     |


        **📤 Returns:**

        | Type              | Description                                                                |
        |-------------------|----------------------------------------------------------------------------|
        | `CompositeOutput` | A `CompositeOutput` containing resource outputs                            |

    === "C FFI"

        **🛠️ Signature:**
        ```c
        typedef struct pulumi_context_t pulumi_context_t;
        typedef struct pulumi_output_t pulumi_output_t;

        typedef struct pulumi_object_field_t {
            const char *name;
            const struct pulumi_output_t *value;
        } pulumi_object_field_t;

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

#### Get config

!!! abstract "Receives value from [config](https://www.pulumi.com/docs/iac/concepts/config/)"

    === "Wasm"

        **🛠️ Signature:**
        ```wit
        package component:pulumi-gestalt@0.0.0-DEV;

        interface types {
            variant config-value {
                plaintext(string),
                secret(output),
            }
        }

        interface context {
            use types.config-value;

            resource context {
                get-config: func(name: option<string>, key: string) -> option<config-value>;
            }
        }
        ```

        **config-value**

        | Name      | Type              | Description         |
        |-----------|-------------------|---------------------|
        | plaintext | string            | Config value if it is not secret       |
        | secret    | output            | Config value hidden in output if it is a secret      |


        **📥 Parameters:**

        | Name      | Type              | Description                                                       |
        |-----------|-------------------|-------------------------------------------------------------------|
        | `name`    | `option<string>`  | Config namespace |
        | `key`     | `string`          | Config key                                       |


        **📤 Returns:**

        | Type             | Description                                                                                                       |
        |------------------|-------------------------------------------------------------------------------------------------------------------|
        | `Option[ConfigValue]` | None if config does not exist, String if it is plaintext, String hidden in output if it is secret |

    === "Rust"

        **🛠️ Signature:**
        [docs.rs](https://docs.rs/pulumi_gestalt_rust/latest/pulumi_gestalt_rust/type.Context.html#method.get_config)
        ```rust
        pub type Context = NativeContext;

        impl GestaltContext for NativeContext {
            fn get_config(
                &self,
                name: Option<&str>,
                key: &str,
            ) -> Option<ConfigValue<Self::Output<String>>> {  }
        }

        pub enum ConfigValue<T> {
            PlainText(String),
            Secret(T),
        }
        ```

        **📥 Parameters:**

        | Name      | Type              | Description                                                       |
        |-----------|-------------------|-------------------------------------------------------------------|
        | `self`    | `&Context`        | Instance of context                                               |
        | `name`    | `Option<&str>`    | Config namespace                                                  |
        | `key`     | `&str`            | Config key                                                        |


        **📤 Returns:**

        | Type                        | Description                                                                |
        |-----------------------------|----------------------------------------------------------------------------|
        | `Option<ConfigValue<Output<String>>>` | None if config does not exist, ConfigValue enum with value or secret output |

    === "C FFI"

        **🛠️ Signature:**
        ```c
        typedef struct pulumi_context_t pulumi_context_t;

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

#### Map

!!! abstract "Applies a function to transform the value inside an `Output`"

    === "Wasm"

        **🛠️ Signature:**
        ```wit
        package component:pulumi-gestalt@0.0.0-DEV;

        interface output-interface {
            resource output {
                map: func(function-name: string) -> output;
            }
        }
        ```

        **📥 Parameters:**

        | Name            | Type     | Description                            |
        |-----------------|----------|----------------------------------------|
        | `function-name` | `string` | Name of the function to apply          |

        **📤 Returns:**

        | Type     | Description                               |
        |----------|-------------------------------------------|
        | `Output` | An `Output` containing transformed value. |

    === "Rust"

        **🛠️ Signature:**
        ```rust
        pub struct Output<T> { /* private fields */ }

        fn map<B, F>(&self, f: F) -> Output<B>
        where
            F: Fn(T) -> B + Send + 'static,
            T: DeserializeOwned,
            B: Serialize, {}
        ```

        **📥 Parameters:**

        | Name     | Type     | Description                       |
        |----------|----------|-----------------------------------|
        | `output` | `Output` | An `Output` object to transform   |
        | `func`   | `T -> B` | Function to apply to the `Output` |

        **📤 Returns:**

        | Type     | Description                               |
        |----------|-------------------------------------------|
        | `Output` | An `Output` containing transformed value. |

    === "C FFI"

        **🛠️ Signature:**
        ```c
        typedef struct pulumi_context_t pulumi_context_t;

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

#### Clone

!!! abstract "Creates a copy of an `Output`"

    === "Wasm"
    
        **🛠️ Signature:**
        ```wit
        package component:pulumi-gestalt@0.0.0-DEV;
    
        interface output-interface {
            resource output {
                clone: func() -> output;
            }
        }
        ```
    
        **📥 Parameters:**
    
        None
    
        **📤 Returns:**
    
        | Type     | Description                                                                                                       |
        |----------|-------------------------------------------------------------------------------------------------------------------|
        | `Output` | A copy of the `Output`. Does not have to be freed. It will be freed automatically when destroying context |
    
    === "Rust"
    
        Output<T> implements `Clone` trait, so you can use `clone()` method to create a copy of it.
    
    === "C FFI"
    
        Does not exist in C FFI. Since `Output` is a reference type, it is not necessary to clone it. You can use the same `Output` instance in multiple places without cloning. Cleanup happens automatically when the context is destroyed.

#### Combine

!!! abstract "Combines multiple `Output` objects to create a single composite `Output`"

    === "Wasm"

        **🛠️ Signature:**
        ```wit
        package component:pulumi-gestalt@0.0.0-DEV;

        interface output-interface {
            resource output {
                combine: func(outputs: list<borrow<output>>) -> output;
            }
        }
        ```

        **📥 Parameters:**

        | Name      | Type                    | Description                                    |
        |-----------|-------------------------|------------------------------------------------|
        | `outputs` | `list<borrow<output>>` | List of `Output` objects to combine into `this` |

        **📤 Returns:**

        | Type     | Description                                                                                                       |
        |----------|-------------------------------------------------------------------------------------------------------------------|
        | `Output` | An `Output` containing combined values. The structure looks like `[output, outputs[0], outputs[1], ...]` Does not have to be freed. It will be freed automatically when destroying context |

    === "Rust"

        **🛠️ Signature:**
        [docs.rs](https://docs.rs/pulumi_gestalt_rust_integration/latest/pulumi_gestalt_rust_integration/struct.Output.html#method.combine)
        ```rust
        pub struct Output { /* private fields */ }

        impl Output {
            pub fn combine(&self, others: &[&Output]) -> Output {}
        }
        ```

        **📥 Parameters:**

        | Name      | Type           | Description                                     |
        |-----------|----------------|-------------------------------------------------|
        | `others`  | `&[&Output]`   | List of `Output` objects to combine into `self` |

        **📤 Returns:**

        | Type            | Description                                                                 |
        |-----------------|-----------------------------------------------------------------------------|
        | `Output<RESULT>` | An `Output` containing combined values from all the provided outputs       |

    === "C FFI"

        **🛠️ Signature:**
        ```c
        typedef struct pulumi_output_t pulumi_output_t;

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

#### Add to export

!!! abstract "Add output as [stack output](https://www.pulumi.com/tutorials/building-with-pulumi/stack-outputs/)."

    === "Wasm"

        **🛠️ Signature:**
        ```wit
        package component:pulumi-gestalt@0.0.0-DEV;

        interface output-interface {
            resource output {
                add-to-export: func(name: string);
            }
        }
        ```

        **📥 Parameters:**

        | Name   | Type     | Description              |
        |--------|----------|--------------------------|
        | `name` | `string` | Name of the stack output |

    === "Rust"

        **🛠️ Signature:**
        ```rust
        pub struct Output { /* private fields */ }

        impl Output {
            pub fn add_export(&self, name: String)
        }
        ```

        **📥 Parameters:**

        | Name     | Type     | Description                            |
        |----------|----------|----------------------------------------|
        | `self`   | `&Output` | `Output` object to add as stack output |
        | `name`   | `String` | Name of the stack output               |

    === "C FFI"

        **🛠️ Signature:**
        ```c
        typedef struct pulumi_output_t pulumi_output_t;

        void pulumi_output_add_to_export(const struct pulumi_output_t *value, const char *name);
        ```

        **📥 Parameters:**

        | Name    | Type                     | Description                            |
        |---------|--------------------------|----------------------------------------|
        | `value` | `const pulumi_output_t*` | `Output` object to add as stack output |
        | `name`  | `const char*`            | Name of the stack output               |

### CompositeOutput

This is a special type of `Output` that represents the result of a resource operation. It contains multiple fields, each
of which can be accessed individually.

#### Get field

!!! abstract "Get resource operation result value"

    === "Wasm"

        **🛠️ Signature:**
        ```wit
        package component:pulumi-gestalt@0.0.0-DEV;

        interface output-interface {
            resource composite-output {
                get-field: func(field-name: string) -> output;
            }
        }
        ```

        **📥 Parameters:**

        | Name         | Type     | Description |
        |--------------|----------|-------------|
        | `field-name` | `string` | Field name  |

        **📤 Returns:**

        | Type     | Description                                                                                                       |
        |----------|-------------------------------------------------------------------------------------------------------------------|
        | `Output` | An `Output` containing field value. Does not have to be freed. It will be freed automatically when destroying context |

    === "Rust"

        **🛠️ Signature:**
        ```rust
        pub struct CompositeOutput { /* private fields */ }

        impl CompositeOutput {
            pub fn get_field(&self, field_name: String) -> Output
        }
        ```

        **📥 Parameters:**

        | Name         | Type              | Description                            |
        |--------------|-------------------|----------------------------------------|
        | `self`       | `&CompositeOutput` | Composite output returned from resource operation |
        | `field_name` | `String`          | Field name                             |

        **📤 Returns:**

        | Type     | Description                                                                                                       |
        |----------|-------------------------------------------------------------------------------------------------------------------|
        | `Output` | An `Output` containing field value. Does not have to be freed. It will be freed automatically when destroying context |

    === "C FFI"

        **🛠️ Signature:**
        ```c
        typedef struct pulumi_output_t pulumi_output_t;
        typedef struct pulumi_composite_output_t pulumi_composite_output_t;

        struct pulumi_output_t *pulumi_composite_output_get_field(struct pulumi_composite_output_t *output,
                                                                  const char *field_name);
        ```

        **📥 Parameters:**

        | Name         | Type                        | Description                            |
        |--------------|----------------------------|----------------------------------------|
        | `output`     | `pulumi_composite_output_t*` | Composite output returned from resource operation  |
        | `field_name` | `const char*`               | Field name                             |

        **📤 Returns:**

        | Type               | Description                                                                                                       |
        |--------------------|-------------------------------------------------------------------------------------------------------------------|
        | `pulumi_output_t*` | An `Output` containing field value. Does not have to be freed. It will be freed automatically when destroying context |


### Get Schema

!!! abstract "Returns schema for the provider"

    === "Wasm"

        _Not applicable. This function is not available in the Wasm integration._

    === "Rust"

        **🛠️ Signature:**
        [docs.rs](https://docs.rs/pulumi_gestalt_rust_integration/latest/pulumi_gestalt_rust_integration/fn.get_schema.html)
        ```rust
        pub fn get_schema(
            provider_name: &str,
            provider_version: &str,
            modules: Option<&[&str]>,
        ) -> Result<model::Package>
        ```

        **📥 Parameters:**

        | Name               | Type             | Description                                                                 |
        |--------------------|------------------|-----------------------------------------------------------------------------|
        | `provider_name`    | `&str`           | Name of the provider (e.g. "aws", "azure")                                  |
        | `provider_version` | `&str`           | Version of the provider (e.g. "5.0.0")                                     |
        | `modules`          | `Option<&[&str]>` | Optional list of modules to include. If `None`, all modules are included. |

        **📤 Returns:**

        | Type                | Description                                         |
        |---------------------|-----------------------------------------------------|
        | `Result<model::Package>` | The provider schema or an error if not found. |

    === "C FFI"

        **🛠️ Signature:**
        ```c
        /**
         * String that may contain nulls and is not null-terminated.
         */
        typedef struct pulumi_string_t {
          uint8_t *data;
          uintptr_t len;
        } pulumi_string_t;

        void pulumi_string_free(struct pulumi_string_t *value);

        /**
         * Returns protobuf encoded schema for the provider.
         * Modules for provider can be found in Pulumi registry on left side with (M) icon:
         * - [AWS](https://www.pulumi.com/registry/packages/aws/)
         * - [Azure](https://www.pulumi.com/registry/packages/azure/)
         * - [GCP](https://www.pulumi.com/registry/packages/gcp/)
         *
         * Empty modules list means that no modules are used.
         * To use all modules, pass null for the modules pointer and 0 for the size.
         */
        struct pulumi_string_t *pulumi_get_schema(const char *provider_name,
                                                  const char *provider_version,
                                                  const char *const *modules,
                                                  uintptr_t modules_size);

        ```

        **📥 Parameters:**

        | Name               | Type                | Description                                                                                           |
        |--------------------|---------------------|-------------------------------------------------------------------------------------------------------|
        | `provider_name`    | `const char*`       | Name of the provider (e.g. "aws", "azure")                                                            |
        | `provider_version` | `const char*`       | Version of the provider (e.g. "5.0.0")                                                               |
        | `modules`          | `const char* const*`| Array of module names to include. Pass `NULL` to include all modules.                                 |
        | `modules_size`     | `uintptr_t`         | Size of the `modules` array. Pass `0` if `modules` is `NULL`.                                           |

        **📤 Returns:**

        | Type                    | Description                                                                                                                               |
        |-------------------------|-------------------------------------------------------------------------------------------------------------------------------------------|
        | `struct pulumi_string_t*` | A `pulumi_string_t` struct containing the protobuf encoded schema. Must be freed with `pulumi_string_free` when no longer needed.        |


### Abstraction Levels for `Output::map`

The `Output::map` function is implemented at different levels of abstraction depending on the language or integration:

#### High Level

- **Description**: Pass a function of type `T1 -> T2` to `Output::map`.
- **Examples**: [Rust language](../languages/rust/index.md).

#### Medium Level

- **Description**: Pass a function of type `String -> String` to `Output::map` where `String` is a serialized JSON
  value.
- **Examples**: [C FFI](c-ffi.md), [Rust integration](rust.md).

#### Low Level

- **Description**: Pass a function id to `Output::map` and receive it later.
- **Examples**: [Wasm](wasm.md).
