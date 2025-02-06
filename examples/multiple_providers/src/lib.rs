use anyhow::Result;
use pulumi_gestalt_providers_docker::container;
use pulumi_gestalt_providers_docker::container::ContainerArgs;
use pulumi_gestalt_providers_random::random_string;
use pulumi_gestalt_providers_random::random_string::RandomStringArgs;
use pulumi_gestalt_rust::{add_export, pulumi_main};
use pulumi_gestalt_rust::{Output, PulumiContext};

pulumi_main!();

fn pulumi_main(context: &PulumiContext) -> Result<()> {
    let length: Output<i32> = Output::new(context, &12).map(|i: i32| i * 3);
    let random_string = random_string::create(
        context,
        "test",
        RandomStringArgs::builder().length(length).build_struct(),
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

    add_export("logs", &cont.container_logs);
    add_export("result", &random_string.result);
    add_export("transformed_result", &t);
    add_export("number", &number);
    Ok(())
}
