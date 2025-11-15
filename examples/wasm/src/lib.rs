use pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::context::Context;
use pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::types::{
    ConfigValue, FunctionInvocationResult,
};
use pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::types::{
    FunctionInvocationRequest, ObjectField, RegisterResourceRequest, ResourceInvokeRequest,
};

fn generate_random_value(ctx: &Context) {
    let output = ctx.create_output("16", false);
    let inputs = vec![ObjectField {
        name: "length".to_string(),
        value: &output,
    }];

    let register_resource_request = RegisterResourceRequest {
        type_: "random:index/randomString:RandomString".to_string(),
        name: "my_name".to_string(),
        version: "4.15.1".to_string(),
        object: inputs,
    };

    let composite_output = ctx.register_resource(&register_resource_request);
    let output_result = composite_output.get_field("result");
    output_result.add_to_export("result");
}

fn run_command(ctx: &Context) {
    let output = ctx.create_output("\"whoami\"", false);

    let inputs = vec![ObjectField {
        name: "command".to_string(),
        value: &output,
    }];

    let register_resource_request = ResourceInvokeRequest {
        token: "command:local:run".to_string(),
        version: "1.0.2".to_string(),
        object: inputs,
    };

    let compose_output = ctx.invoke_resource(&register_resource_request);

    let stdout_output = compose_output.get_field("stdout");

    stdout_output.add_to_export("whoami_stdout");
}

fn perform_operations_on_outputs(ctx: &Context) {
    let output = ctx.create_output("16", false);

    let output_2 = output.map("double");
    let output_3 = output_2.map("static");
    let output_4 = output.combine(&[&output_2, &output_3]);

    output_2.add_to_export("double_length");
    output_3.add_to_export("static_string");
    output_4.add_to_export("combined");
}

fn perform_operations_on_default_config(ctx: &Context) {
    if ctx.get_config(None, "test").is_some() {
        panic!("NULL was expected but not returned");
    }

    let plaintext = ctx
        .get_config(None, "plaintext")
        .expect("Expected plaintext value");
    if let ConfigValue::Plaintext(plain_value) = plaintext {
        if plain_value != "plain_value" {
            panic!(
                "plain_value was expected but not returned. Returned value is [{}]",
                plain_value
            );
        }
    } else {
        panic!("PlainText tag was expected but not returned");
    }

    let secret = ctx
        .get_config(None, "secret")
        .expect("Expected secret value");
    if let ConfigValue::Secret(secret_output) = secret {
        secret_output.add_to_export("secret");
    } else {
        panic!("Secret tag was expected but not returned");
    }
}

fn perform_operations_on_custom_config(ctx: &Context) {
    if ctx.get_config(Some("namespace"), "test").is_some() {
        panic!("NULL was expected but not returned");
    }

    let plaintext = ctx
        .get_config(Some("namespace"), "plaintext")
        .expect("Expected plaintext value");
    if let ConfigValue::Plaintext(plain_value) = plaintext {
        if plain_value != "plain_value_namespace" {
            panic!(
                "plain_value_namespace was expected but not returned. Returned value is [{}]",
                plain_value
            );
        }
    } else {
        panic!("PlainText tag was expected but not returned");
    }

    let secret = ctx
        .get_config(Some("namespace"), "secret")
        .expect("Expected secret value");
    if let ConfigValue::Secret(secret_output) = secret {
        secret_output.add_to_export("secret_namespace");
    } else {
        panic!("Secret tag was expected but not returned");
    }
}

#[cfg(target_arch = "wasm32")]
#[unsafe(export_name = "component:pulumi-gestalt/pulumi-main@0.0.0-DEV#main")]
unsafe extern "C" fn __wasm_main() {
    main();
}

fn main() {
    let ctx = Context::new();
    generate_random_value(&ctx);
    run_command(&ctx);
    perform_operations_on_outputs(&ctx);
    perform_operations_on_default_config(&ctx);
    perform_operations_on_custom_config(&ctx);

    run_loop(&ctx);
}

fn run_loop(context: &Context) {
    let mut function_requests = context.finish(&[]);

    loop {
        if function_requests.is_empty() {
            return;
        }
        let function_results: Vec<_> = function_requests
            .iter()
            .map(
                |FunctionInvocationRequest {
                     id,
                     function_name,
                     value,
                 }| {
                    if function_name == "double" {
                        let v: i32 = value.parse().unwrap();
                        let result = (v * 2).to_string();
                        FunctionInvocationResult { id, value: result }
                    } else if function_name == "static" {
                        let result = "\"my_string\"".to_string();
                        FunctionInvocationResult { id, value: result }
                    } else {
                        panic!("Unknown function name: `{}`", function_name);
                    }
                },
            )
            .collect();

        function_requests = context.finish(&function_results);
    }
}
