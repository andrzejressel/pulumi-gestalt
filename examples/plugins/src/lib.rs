use pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::context::Context;

#[cfg(target_arch = "wasm32")]
#[unsafe(export_name = "component:pulumi-gestalt/pulumi-main@0.0.0-DEV#main")]
unsafe extern "C" fn __wasm_main() {
    main();
}

fn main() {
    let ctx = Context::new();
    ctx.finish(&[]);
}

#[unsafe(link_section = "pulumi_gestalt_provider::random")]
#[unsafe(no_mangle)]
#[cfg(target_arch = "wasm32")]
static PULUMI_WASM_PROVIDER_RANDOM: [u8; 45] =
    *b"{\"version\":\"4.15.1\",\"pluginDownloadURL\":null}";
