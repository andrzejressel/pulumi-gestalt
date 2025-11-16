use pulumi_gestalt_providers_docker::functions::get_remote_image;
use pulumi_gestalt_providers_docker::functions::get_remote_image::GetRemoteImageArgs;
use pulumi_gestalt_providers_docker::types::{ContainerLabel, DockerBuild};
use pulumi_gestalt_providers_docker::{container, image};
use pulumi_gestalt_rust::GestaltOutput;
use pulumi_gestalt_rust::{Context, add_export};

pulumi_gestalt_rust::pulumi_main!();

fn pulumi_main(context: &Context) -> anyhow::Result<()> {
    let cont = container::create(
        context,
        "container",
        container::ContainerArgs::builder()
            .attach(true)
            .command(["echo", "Hello World!"])
            .image("public.ecr.aws/ubuntu/ubuntu:latest")
            .labels(vec![
                ContainerLabel::builder()
                    .label("label_1")
                    .value("value_1")
                    .build_struct(),
            ])
            .logs(true)
            .must_run(false)
            .build_struct(),
    );

    let image = image::create(
        context,
        "image",
        image::ImageArgs::builder()
            .build(DockerBuild::builder().context("docker/").build_struct())
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
