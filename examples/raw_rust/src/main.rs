use pulumi_gestalt_rust_integration::{
    ConfigValue, InvokeResourceRequest, RegisterResourceRequest, get_schema,
};
use serde_json::{Value, json};
use std::collections::HashMap;

type Context = pulumi_gestalt_rust_integration::Context<Box<dyn Fn(Value) -> Value>>;

async fn generate_random_value(ctx: &Context) {
    let output = ctx.create_output(json!(16), false);

    let register_resource_request = RegisterResourceRequest {
        r#type: "random:index/randomString:RandomString".to_string(),
        name: "my_name".to_string(),
        version: "4.15.1".to_string(),
        inputs: HashMap::from([("length".into(), output.clone())]),
    };

    let composite_output = ctx.register_resource(register_resource_request).await;
    let output_result = composite_output.get_field("result".into()).await;
    output_result.add_export("result".into()).await;
}

async fn run_command(ctx: &Context) {
    let output = ctx.create_output(json!("whoami"), false);

    let register_resource_request = InvokeResourceRequest {
        token: "command:local:run".to_string(),
        version: "1.0.2".to_string(),
        inputs: HashMap::from([("command".into(), output)]),
    };

    let compose_output = ctx.invoke_resource(register_resource_request).await;

    let stdout_output = compose_output.get_field("stdout".into()).await;

    stdout_output.add_export("whoami_stdout".into()).await;
}

async fn perform_operations_on_outputs(ctx: &Context) {
    let output = ctx.create_output(json!(16), false);

    let output_2 = output
        .map(Box::new(|s| {
            let i = s.as_i64().unwrap();
            (i * 2).to_string().into()
        }))
        .await;
    let output_3 = output_2.map(Box::new(|_| json!("my_string"))).await;

    let output_4 = output.combine(&[&output_2, &output_3]).await;

    output_2.add_export("double_length".into()).await;
    output_3.add_export("static_string".into()).await;
    output_4.add_export("combined".into()).await;
}

async fn perform_operations_on_default_config(ctx: &Context) {
    if ctx.get_config_value(None, "test").await.is_some() {
        panic!("NULL was expected but not returned");
    }

    let plaintext = ctx
        .get_config_value(None, "plaintext")
        .await
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
        .await
        .expect("Expected secret value");
    if let ConfigValue::Secret(secret_output) = secret {
        secret_output.add_export("secret".into()).await;
    } else {
        panic!("Secret tag was expected but not returned");
    }
}

async fn perform_operations_on_custom_config(ctx: &Context) {
    if ctx
        .get_config_value(Some("namespace"), "test")
        .await
        .is_some()
    {
        panic!("NULL was expected but not returned");
    }

    let plaintext = ctx
        .get_config_value(Some("namespace"), "plaintext")
        .await
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
        .await
        .expect("Expected secret value");
    if let ConfigValue::Secret(secret_output) = secret {
        secret_output.add_export("secret_namespace".into()).await;
    } else {
        panic!("Secret tag was expected but not returned");
    }
}

fn obtain_schema() {
    let _ = get_schema("docker", "4.5.3", None).unwrap();
}

#[tokio::main]
async fn main() {
    let ctx = Context::new().await;

    generate_random_value(&ctx).await;
    run_command(&ctx).await;
    perform_operations_on_outputs(&ctx).await;
    perform_operations_on_default_config(&ctx).await;
    perform_operations_on_custom_config(&ctx).await;
    obtain_schema();

    pulumi_gestalt_rust_integration::finish::finish_lambdas_sequentially(ctx).await;
}
