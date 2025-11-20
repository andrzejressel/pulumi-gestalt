use std::collections::HashMap;
use serde_json::json;
use pulumi_gestalt_rust_integration::get_schema;
use pulumi_gestalt_rust_integration::*;

type Context = pulumi_gestalt_rust_integration::Context<Box<dyn Fn(String) -> String>>;

fn generate_random_value(ctx: &Context) {
    let output = Context::create_output(json!(16), false);

    let register_resource_request = RegisterResourceRequest {
        r#type: "random:index/randomString:RandomString".to_string(),
        name: "my_name".to_string(),
        version: "4.15.1".to_string(),
        inputs: HashMap::from([
            ("length".into(), output.clone())
        ]),
    };

    let composite_output = ctx.register_resource(register_resource_request);
    let output_result = composite_output.get_field("result".into());
    output_result.add_export("result".into());
}

fn run_command(ctx: &Context) {
    let output = Context::create_output(json!("whoami"), false);

    let register_resource_request = InvokeResourceRequest {
        token: "command:local:run".to_string(),
        version: "1.0.2".to_string(),
        inputs: HashMap::from([
            ("command".into(), output)
        ]),
    };

    let compose_output = ctx.invoke_resource(register_resource_request);

    let stdout_output = compose_output.get_field("stdout".into());

    stdout_output.add_export("whoami_stdout".into());
}

fn perform_operations_on_outputs(ctx: &Context) {
    let output = Context::create_output(json!(16), false);

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
