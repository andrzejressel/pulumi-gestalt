use pulumi_gestalt_rust_integration::{
    ConfigValue, Context, InvokeResourceRequest, ObjectField, RegisterResourceRequest, get_schema,
};

fn generate_random_value(ctx: &Context) {
    let output = ctx.create_output("16".to_string(), false);
    let inputs = vec![ObjectField {
        name: "length".to_string(),
        value: &output,
    }];

    let register_resource_request = RegisterResourceRequest {
        type_: "random:index/randomString:RandomString".to_string(),
        name: "my_name".to_string(),
        version: "4.15.1".to_string(),
        inputs: &inputs,
    };

    let composite_output = ctx.register_resource(register_resource_request);
    let output_result = composite_output.get_field("result".to_string());
    output_result.add_export("result".to_string());
}

fn run_command(ctx: &Context) {
    let output = ctx.create_output("\"whoami\"".to_string(), false);

    let inputs = vec![ObjectField {
        name: "command".to_string(),
        value: &output,
    }];

    let register_resource_request = InvokeResourceRequest {
        token: "command:local:run".to_string(),
        version: "1.0.2".to_string(),
        inputs: &inputs,
    };

    let compose_output = ctx.invoke_resource(register_resource_request);

    let stdout_output = compose_output.get_field("stdout".to_string());

    stdout_output.add_export("whoami_stdout".to_string());
}

fn perform_operations_on_outputs(ctx: &Context) {
    let output = ctx.create_output("16".to_string(), false);

    let output_2 = output.map(Box::new(|s| {
        let i = s.parse::<i32>().unwrap();
        (i * 2).to_string()
    }));
    let output_3 = output_2.map(Box::new(|_| "\"my_string\"".to_string()));

    let output_4 = output.combine(&[&output_2, &output_3]);

    output_2.add_export("double_length".to_string());
    output_3.add_export("static_string".to_string());
    output_4.add_export("combined".to_string());
}

fn perform_operations_on_default_config(ctx: &Context) {
    if ctx.get_config_value(None, "test").is_some() {
        panic!("NULL was expected but not returned");
    }

    let plaintext = ctx
        .get_config_value(None, "plaintext")
        .expect("Expected plaintext value");
    if let ConfigValue::PlainText(plain_value) = plaintext {
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
        .get_config_value(None, "secret")
        .expect("Expected secret value");
    if let ConfigValue::Secret(secret_output) = secret {
        secret_output.add_export("secret".to_string());
    } else {
        panic!("Secret tag was expected but not returned");
    }
}

fn perform_operations_on_custom_config(ctx: &Context) {
    if ctx.get_config_value(Some("namespace"), "test").is_some() {
        panic!("NULL was expected but not returned");
    }

    let plaintext = ctx
        .get_config_value(Some("namespace"), "plaintext")
        .expect("Expected plaintext value");
    if let ConfigValue::PlainText(plain_value) = plaintext {
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
        .get_config_value(Some("namespace"), "secret")
        .expect("Expected secret value");
    if let ConfigValue::Secret(secret_output) = secret {
        secret_output.add_export("secret_namespace".to_string());
    } else {
        panic!("Secret tag was expected but not returned");
    }
}

fn obtain_schema() {
    let _ = get_schema("docker", "4.5.3", None).unwrap();
}

fn main() {
    let ctx = Context::create_context();

    generate_random_value(&ctx);
    run_command(&ctx);
    perform_operations_on_outputs(&ctx);
    perform_operations_on_default_config(&ctx);
    perform_operations_on_custom_config(&ctx);
    obtain_schema();

    ctx.finish();
}
