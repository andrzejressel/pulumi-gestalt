#[allow(clippy::all)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_unsafe)]
#[cfg(feature = "client")]
pub mod client_bindings {
    wit_bindgen::generate!({
        world: "client",
        generate_all
    });
}

#[allow(clippy::all)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_unsafe)]
#[cfg(feature = "logger")]
pub mod bindings_logger {
    wit_bindgen::generate!({
        world: "logger",
        generate_all
    });
}

#[allow(clippy::all)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_unsafe)]
#[cfg(feature = "pulumi_gestalt")]
pub mod pulumi_gestalt_bindings {
    wit_bindgen::generate!({
        world: "pulumi-gestalt",
        generate_all,
        pub_export_macro: true
    });
}

#[allow(clippy::all)]
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_unsafe)]
#[cfg(feature = "runner")]
pub mod bindings_runner {
    
    pub struct SingleThreadedContext(pub pulumi_gestalt_rust_integration::Context);
    impl SingleThreadedContext {
        pub fn new(context: pulumi_gestalt_rust_integration::Context) -> Self {
            Self(context)
        }
    }
    unsafe impl Send for SingleThreadedContext {}
    
    pub struct SingleThreadedOutput(pub pulumi_gestalt_rust_integration::Output);
    unsafe impl Send for SingleThreadedOutput {}
    impl SingleThreadedOutput {
        pub fn new(output: pulumi_gestalt_rust_integration::Output) -> Self {
            Self(output)
        }
    }
    
    pub struct SingleThreadedCompositeOutput(pub pulumi_gestalt_rust_integration::CompositeOutput);
    unsafe impl Send for SingleThreadedCompositeOutput {}
    impl SingleThreadedCompositeOutput {
        pub fn new(output: pulumi_gestalt_rust_integration::CompositeOutput) -> Self {
            Self(output)
        }
    }
    
    wasmtime::component::bindgen!({
        world: "client",
        async: false,
        trappable_imports: true,
        with: {
            "component:pulumi-gestalt/context/context":SingleThreadedContext,
            "component:pulumi-gestalt/output-interface/output": SingleThreadedOutput,
            "component:pulumi-gestalt/output-interface/composite-output": SingleThreadedCompositeOutput,
        }
    });
}
