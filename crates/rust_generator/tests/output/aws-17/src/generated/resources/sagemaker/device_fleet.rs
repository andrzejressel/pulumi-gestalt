/// Provides a SageMaker Device Fleet resource.
///
/// ## Example Usage
///
/// ### Basic usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = device_fleet::create(
///         "example",
///         DeviceFleetArgs::builder()
///             .device_fleet_name("example")
///             .output_config(
///                 DeviceFleetOutputConfig::builder()
///                     .s3OutputLocation("s3://${exampleAwsS3Bucket.bucket}/prefix/")
///                     .build_struct(),
///             )
///             .role_arn("${test.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import SageMaker Device Fleets using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:sagemaker/deviceFleet:DeviceFleet example my-fleet
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod device_fleet {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DeviceFleetArgs {
        /// A description of the fleet.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::Input<Option<String>>,
        /// The name of the Device Fleet (must be unique).
        #[builder(into)]
        pub device_fleet_name: pulumi_gestalt_rust::Input<String>,
        /// Whether to create an AWS IoT Role Alias during device fleet creation. The name of the role alias generated will match this pattern: "SageMakerEdge-{DeviceFleetName}".
        #[builder(into, default)]
        pub enable_iot_role_alias: pulumi_gestalt_rust::Input<Option<bool>>,
        /// Specifies details about the repository. see Output Config details below.
        #[builder(into)]
        pub output_config: pulumi_gestalt_rust::Input<
            super::super::types::sagemaker::DeviceFleetOutputConfig,
        >,
        /// The Amazon Resource Name (ARN) that has access to AWS Internet of Things (IoT).
        #[builder(into)]
        pub role_arn: pulumi_gestalt_rust::Input<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::Input<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct DeviceFleetResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) assigned by AWS to this Device Fleet.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// A description of the fleet.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Device Fleet (must be unique).
        pub device_fleet_name: pulumi_gestalt_rust::Output<String>,
        /// Whether to create an AWS IoT Role Alias during device fleet creation. The name of the role alias generated will match this pattern: "SageMakerEdge-{DeviceFleetName}".
        pub enable_iot_role_alias: pulumi_gestalt_rust::Output<Option<bool>>,
        pub iot_role_alias: pulumi_gestalt_rust::Output<String>,
        /// Specifies details about the repository. see Output Config details below.
        pub output_config: pulumi_gestalt_rust::Output<
            super::super::types::sagemaker::DeviceFleetOutputConfig,
        >,
        /// The Amazon Resource Name (ARN) that has access to AWS Internet of Things (IoT).
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DeviceFleetArgs,
    ) -> DeviceFleetResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DeviceFleetArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> DeviceFleetResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DeviceFleetArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> DeviceFleetResult {
        let description_binding = args.description.get_output(ctx);
        let device_fleet_name_binding = args.device_fleet_name.get_output(ctx);
        let enable_iot_role_alias_binding = args.enable_iot_role_alias.get_output(ctx);
        let output_config_binding = args.output_config.get_output(ctx);
        let role_arn_binding = args.role_arn.get_output(ctx);
        let tags_binding = args.tags.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:sagemaker/deviceFleet:DeviceFleet".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deviceFleetName".into(),
                    value: &device_fleet_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableIotRoleAlias".into(),
                    value: &enable_iot_role_alias_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "outputConfig".into(),
                    value: &output_config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        DeviceFleetResult {
            id: o.get_id(),
            urn: o.get_urn(),
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            device_fleet_name: o.get_field("deviceFleetName"),
            enable_iot_role_alias: o.get_field("enableIotRoleAlias"),
            iot_role_alias: o.get_field("iotRoleAlias"),
            output_config: o.get_field("outputConfig"),
            role_arn: o.get_field("roleArn"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
