#[allow(clippy::all)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_unsafe)]
#[cfg(feature = "client")]
pub mod client_bindings {
    wit_bindgen::generate!({
        world: "pulumi-gestalt",
        generate_all
    });
}

#[allow(clippy::all)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_unsafe)]
#[cfg(feature = "runner")]
pub mod bindings_runner {
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::sync::Arc;

    type FunctionType = String;

    pub struct SingleThreadedContext {
        pub engine: pulumi_gestalt_rust_integration::Context<FunctionType>,
    }

    impl SingleThreadedContext {
        pub fn new(engine: pulumi_gestalt_rust_integration::Context<FunctionType>) -> Self {
            Self { engine: engine }
        }
    }

    pub struct SingleThreadedOutput {
        pub output: pulumi_gestalt_rust_integration::Output<FunctionType>,
    }
    impl SingleThreadedOutput {
        pub fn new(output: pulumi_gestalt_rust_integration::Output<FunctionType>) -> Self {
            Self { output }
        }
    }

    pub struct SingleThreadedCompositeOutput {
        pub output: pulumi_gestalt_rust_integration::RegisterResourceOutput<FunctionType>,
    }
    impl SingleThreadedCompositeOutput {
        pub fn new(
            output: pulumi_gestalt_rust_integration::RegisterResourceOutput<FunctionType>,
        ) -> Self {
            Self { output }
        }
    }

    wasmtime::component::bindgen!({
        world: "pulumi-gestalt",
        imports: {
            default: async | trappable
            // "component": async | trappable
        },
        with: {
            "component:pulumi-gestalt/context/context":SingleThreadedContext,
            "component:pulumi-gestalt/output-interface/output": SingleThreadedOutput,
            "component:pulumi-gestalt/output-interface/composite-output": SingleThreadedCompositeOutput,
        }
    });
}
