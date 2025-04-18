/// Provides an GameLift Build resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let test = build::create(
///         "test",
///         BuildArgs::builder()
///             .name("example-build")
///             .operating_system("WINDOWS_2012")
///             .storage_location(
///                 BuildStorageLocation::builder()
///                     .bucket("${testAwsS3Bucket.id}")
///                     .key("${testAwsS3Object.key}")
///                     .roleArn("${testAwsIamRole.arn}")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import GameLift Builds using the ID. For example:
///
/// ```sh
/// $ pulumi import aws:gamelift/build:Build example <build-id>
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod build {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BuildArgs {
        /// Name of the build
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Operating system that the game server binaries are built to run on. Valid values: `WINDOWS_2012`, `AMAZON_LINUX`, `AMAZON_LINUX_2`, `WINDOWS_2016`, `AMAZON_LINUX_2023`.
        #[builder(into)]
        pub operating_system: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Information indicating where your game build files are stored. See below.
        #[builder(into)]
        pub storage_location: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::gamelift::BuildStorageLocation,
        >,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// Version that is associated with this build.
        #[builder(into, default)]
        pub version: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct BuildResult {
        /// GameLift Build ARN.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Name of the build
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Operating system that the game server binaries are built to run on. Valid values: `WINDOWS_2012`, `AMAZON_LINUX`, `AMAZON_LINUX_2`, `WINDOWS_2016`, `AMAZON_LINUX_2023`.
        pub operating_system: pulumi_gestalt_rust::Output<String>,
        /// Information indicating where your game build files are stored. See below.
        pub storage_location: pulumi_gestalt_rust::Output<
            super::super::types::gamelift::BuildStorageLocation,
        >,
        /// Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// Version that is associated with this build.
        pub version: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BuildArgs,
    ) -> BuildResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let operating_system_binding = args.operating_system.get_output(context);
        let storage_location_binding = args.storage_location.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let version_binding = args.version.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:gamelift/build:Build".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "operatingSystem".into(),
                    value: &operating_system_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "storageLocation".into(),
                    value: &storage_location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "version".into(),
                    value: &version_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        BuildResult {
            arn: o.get_field("arn"),
            name: o.get_field("name"),
            operating_system: o.get_field("operatingSystem"),
            storage_location: o.get_field("storageLocation"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            version: o.get_field("version"),
        }
    }
}
