use anyhow::Result;
use pulumi_gestalt_providers_docker::container;
use pulumi_gestalt_providers_docker::container::ContainerArgs;
use pulumi_gestalt_providers_random::random_string;
use pulumi_gestalt_providers_random::random_string::RandomStringArgs;
use pulumi_gestalt_providers_random::Provider;
use pulumi_gestalt_rust::Output;
use pulumi_gestalt_rust::{
    Context, CustomResourceOptions, Provider as ProviderTrait, add_export,
};
use std::rc::Rc;

fn main() {
    pulumi_gestalt_rust::run(pulumi_main).unwrap();
}

fn pulumi_main(context: &Context) -> Result<()> {
    // 1. Create a custom provider
    let provider = Provider::random(
        context,
        "custom-provider",
        Provider::ProviderArgs::builder().build_struct(),
    );

    let length: Output<i32> = context.new_output(&12).map(|i: i32| i * 3);

    // 2. Use the provider in a resource
    let random_string = random_string::create_with_options(
        context,
        "test",
        RandomStringArgs::builder().length(length).build_struct(),
        Some(CustomResourceOptions {
            provider: Some(Rc::new(provider.clone())),
        }),
    );

    // Tests preview behaviour for unknown fields
    let t = random_string.result.map(|s| format!("Result: {s}"));

    // Tests number mapping
    let number = random_string.min_upper.map(|i| i * 2);

    let cont = container::create(
        context,
        "container",
        ContainerArgs::builder()
            .attach(true)
            .command(["echo", "Hello World!"])
            .image("public.ecr.aws/ubuntu/ubuntu:latest")
            .logs(true)
            .must_run(false)
            .build_struct(),
    );

    add_export("test_provider_urn", &provider.get_provider_id());
    add_export("logs", &cont.container_logs);
    add_export("result", &random_string.result);
    add_export("transformed_result", &t);
    add_export("number", &number);
    Ok(())
}
