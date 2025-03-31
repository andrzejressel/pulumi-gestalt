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
    wasmtime::component::bindgen!({
        world: "client",
        async: false,
        trappable_imports: true,
        with: {
            "component:pulumi-gestalt/context/context": pulumi_gestalt_rust_integration::Context,
            "component:pulumi-gestalt/output-interface/output": pulumi_gestalt_rust_integration::Output,
            "component:pulumi-gestalt/output-interface/composite-output": pulumi_gestalt_rust_integration::CompositeOutput,
        }
    });
}
