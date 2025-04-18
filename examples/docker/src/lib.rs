use pulumi_gestalt_providers_docker::functions::get_remote_image;
use pulumi_gestalt_providers_docker::functions::get_remote_image::GetRemoteImageArgs;
use pulumi_gestalt_providers_docker::types::{ContainerLabel, DockerBuild};
use pulumi_gestalt_providers_docker::{container, image};
use pulumi_gestalt_rust::GestaltOutput;
use pulumi_gestalt_rust::{Context, add_export};

#[cfg(target_arch = "wasm32")]
pulumi_gestalt_rust::pulumi_main!();
#[allow(dead_code)]
fn pulumi_main(context: &Context) -> anyhow::Result<()> {
    let cont = container::create(
        context,
        "container",
        container::ContainerArgs::builder()
            .attach(true)
            .command(["echo", "Hello World!"])
            .image("public.ecr.aws/ubuntu/ubuntu:latest")
            .labels(vec![ContainerLabel {
                label: Box::new("label_1".to_string()),
                value: Box::new("value_1".to_string()),
            }])
            .logs(true)
            .must_run(false)
            .build_struct(),
    );

    let image = image::create(
        context,
        "image",
        image::ImageArgs::builder()
            .build(
                DockerBuild::builder()
                    .context(Some("docker/".to_string()))
                    .build_struct(),
            )
            .image_name("image:test")
            .skip_push(true)
            .build_struct(),
    );

    let remote_image = get_remote_image::invoke(
        context,
        GetRemoteImageArgs::builder()
            .name("public.ecr.aws/ubuntu/ubuntu:latest")
            .build_struct(),
    );

    add_export("logs", &cont.container_logs);
    add_export("image_id", &image.image_name);
    add_export("labels", &cont.labels.map(|f| f[0].value.clone()));
    add_export("repo_digest", &remote_image.repo_digest);
    Ok(())
}
