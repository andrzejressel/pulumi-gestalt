/// `Image` builds a Docker image and pushes it Docker and OCI compatible registries.
/// This resource enables running Docker builds as part of a Pulumi deployment.
///
/// Note: This resource does not delete tags, locally or remotely, when destroyed.
///
/// ## Image name
///
/// The Image resource uses `imageName` to refer to a fully qualified Docker image name, by the format `repository:tag`.
/// Note that this does not include any digest information and thus will not cause any updates when passed to dependencies,
/// even when using `latest` tag. To trigger such updates, e.g. when referencing pushed images in container orchestration
/// and management resources, please use the `repoDigest` Output instead, which is of the format
/// `repository@<algorithm>:<hash>` and unique per build/push.
/// As of Docker v4.4, `repoDigest` is now available for local Images.
///
/// ## Cross-platform builds
///
/// The Image resource supports cross-platform builds when the [Docker engine has cross-platform support enabled via emulators](https://docs.docker.com/build/building/multi-platform/#building-multi-platform-images).
/// The Image resource currently supports providing only a single operating system and architecture in the `platform` field, e.g.: `linux/amd64`.
/// To enable this support, you may need to install the emulators in the environment running your Pulumi program.
///
/// If you are using Linux, you may be using Docker Engine or Docker Desktop for Linux, depending on how you have installed Docker. The [FAQ for Docker Desktop for Linux](https://docs.docker.com/desktop/faqs/linuxfaqs/#context) describes the differences and how to select which Docker context is in use.
///
/// * For local development using Docker Desktop, this is enabled by default.
/// * For systems using Docker Engine, install the QEMU binaries and register them with using the docker image from [github.com/tonistiigi/binfmt](https://github.com/tonistiigi/binfmt):
///
///   ```shell
///   docker run --privileged --rm tonistiigi/binfmt --install all
///   ```
/// * In a GitHub Actions workflow, the [docker/setup-qemu-action](https://github.com/docker/setup-qemu-action) can be used instead by adding this step to your workflow file. Example workflow usage:
///
///   ```yaml
///   name: Pulumi
///   on:
///     push:
///       branches:
///         - master
///   jobs:
///     up:
///       name: Preview
///       runs-on: ubuntu-latest
///       steps:
///           # This is the step added:
///         - name: Set up QEMU
///           uses: docker/setup-qemu-action@v2
///           # The ordinary pulumi/actions workflow:
///         - uses: actions/checkout@v3
///         - uses: pulumi/actions@v4
///           with:
///             command: preview
///             stack-name: org-name/stack-name
///           env:
///             PULUMI_ACCESS_TOKEN: ${{ secrets.PULUMI_ACCESS_TOKEN }}
///   ```
///
///
/// ## Example Usage
/// ### A Docker image build
///
/// ```yaml
/// config: {}
/// description: A Docker image build
/// name: image-yaml
/// outputs:
///     imageName: ${demo-image.imageName}
/// resources:
///     demo-image:
///         options:
///             version: v4.4.0
///         properties:
///             build:
///                 context: .
///                 dockerfile: Dockerfile
///                 platform: linux/amd64
///             imageName: username/image:tag1
///             skipPush: true
///         type: docker:Image
/// runtime: yaml
/// variables: {}
/// ```
/// ### A Docker image build and push
///
/// ```yaml
/// config: {}
/// description: A Docker image build and push
/// name: image-push-yaml
/// outputs:
///     imageName: ${demo-push-image.imageName}
///     repoDigest: ${demo-push-image.repoDigest}
/// resources:
///     demo-push-image:
///         options:
///             version: v4.4.0
///         properties:
///             build:
///                 context: .
///                 dockerfile: Dockerfile
///             imageName: docker.io/username/push-image:tag1
///         type: docker:Image
/// runtime: yaml
/// variables: {}
/// ```
/// ### Docker image build using caching with AWS Elastic Container Registry
///
/// ```yaml
/// config: {}
/// description: Docker image build using caching with AWS Elastic Container Registry
/// name: image-caching-yaml
/// outputs:
///     imageName: ${my-app-image.imageName}
/// resources:
///     ecr-repository:
///         properties:
///             name: docker-repository
///         type: aws:ecr:Repository
///     my-app-image:
///         options:
///             version: v4.1.2
///         properties:
///             build:
///                 args:
///                     BUILDKIT_INLINE_CACHE: "1"
///                 cacheFrom:
///                     images:
///                         - ${ecr-repository.repositoryUrl}:latest
///                 context: app/
///                 dockerfile: Dockerfile
///             imageName: ${ecr-repository.repositoryUrl}:latest
///             registry:
///                 password:
///                     fn::secret: ${authToken.password}
///                 server: ${ecr-repository.repositoryUrl}
///         type: docker:Image
/// runtime: yaml
/// variables:
///     authToken:
///         fn::aws:ecr:getAuthorizationToken:
///             registryId: ${ecr-repository.registryId}
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod image {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ImageArgs {
        /// The Docker build context
        #[builder(into, default)]
        pub build: pulumi_gestalt_rust::InputOrOutput<Option<super::types::DockerBuild>>,
        /// A flag to build an image on preview
        #[builder(into, default)]
        pub build_on_preview: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The image name, of the format repository[:tag], e.g. `docker.io/username/demo-image:v1`.
        /// This reference is not unique to each build and push.For the unique manifest SHA of a pushed docker image, or the local image ID, please use `repoDigest`.
        #[builder(into)]
        pub image_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The registry to push the image to
        #[builder(into, default)]
        pub registry: pulumi_gestalt_rust::InputOrOutput<Option<super::types::Registry>>,
        /// A flag to skip a registry push.
        #[builder(into, default)]
        pub skip_push: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
    }
    #[allow(dead_code)]
    pub struct ImageResult {
        /// The fully qualified image name that was pushed to the registry.
        pub base_image_name: pulumi_gestalt_rust::Output<String>,
        /// The path to the build context to use.
        pub context: pulumi_gestalt_rust::Output<String>,
        /// The location of the Dockerfile relative to the docker build context.
        pub dockerfile: pulumi_gestalt_rust::Output<String>,
        /// The fully qualified image name
        pub image_name: pulumi_gestalt_rust::Output<String>,
        /// The image's architecture and OS
        pub platform: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the registry server hosting the image.
        pub registry_server: pulumi_gestalt_rust::Output<String>,
        /// **For pushed images:**
        /// The manifest digest of an image pushed to a registry, of the format repository@<algorithm>:<hash>, e.g. `username/demo-image@sha256:a6ae6dd8d39c5bb02320e41abf00cd4cb35905fec540e37d306c878be8d38bd3`.
        /// This reference is unique per image build and push.
        /// Only available for images pushed to a registry.
        /// Use when passing a reference to a pushed image to container management resources.
        ///
        /// **Local-only images**For local images, this field is the image ID of the built local image, of the format <algorithm>:<hash>, e.g `sha256:826a130323165bb0ccb0374ae774f885c067a951b51a6ee133577f4e5dbc4119`
        pub repo_digest: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ImageArgs,
    ) -> ImageResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let build_binding = args.build.get_output(context);
        let build_on_preview_binding = args.build_on_preview.get_output(context);
        let image_name_binding = args.image_name.get_output(context);
        let registry_binding = args.registry.get_output(context);
        let skip_push_binding = args.skip_push.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "docker:index/image:Image".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "build".into(),
                    value: &build_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "buildOnPreview".into(),
                    value: &build_on_preview_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "imageName".into(),
                    value: &image_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "registry".into(),
                    value: &registry_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "skipPush".into(),
                    value: &skip_push_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ImageResult {
            base_image_name: o.get_field("baseImageName"),
            context: o.get_field("context"),
            dockerfile: o.get_field("dockerfile"),
            image_name: o.get_field("imageName"),
            platform: o.get_field("platform"),
            registry_server: o.get_field("registryServer"),
            repo_digest: o.get_field("repoDigest"),
        }
    }
}
