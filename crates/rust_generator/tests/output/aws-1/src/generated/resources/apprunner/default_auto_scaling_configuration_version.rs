/// Manages the default App Runner auto scaling configuration.
/// When creating or updating this resource the existing default auto scaling configuration will be set to non-default automatically.
/// When creating or updating this resource the configuration is automatically assigned as the default to the new services you create in the future. The new default designation doesn't affect the associations that were previously set for existing services.
/// Each account can have only one default auto scaling configuration per Region.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = auto_scaling_configuration_version::create(
///         "example",
///         AutoScalingConfigurationVersionArgs::builder()
///             .auto_scaling_configuration_name("example")
///             .max_concurrency(50)
///             .max_size(10)
///             .min_size(2)
///             .build_struct(),
///     );
///     let exampleDefaultAutoScalingConfigurationVersion = default_auto_scaling_configuration_version::create(
///         "exampleDefaultAutoScalingConfigurationVersion",
///         DefaultAutoScalingConfigurationVersionArgs::builder()
///             .auto_scaling_configuration_arn("${example.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import App Runner default auto scaling configurations using the current Region. For example:
///
/// ```sh
/// $ pulumi import aws:apprunner/defaultAutoScalingConfigurationVersion:DefaultAutoScalingConfigurationVersion example us-west-2
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod default_auto_scaling_configuration_version {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DefaultAutoScalingConfigurationVersionArgs {
        /// The ARN of the App Runner auto scaling configuration that you want to set as the default.
        #[builder(into)]
        pub auto_scaling_configuration_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DefaultAutoScalingConfigurationVersionResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the App Runner auto scaling configuration that you want to set as the default.
        pub auto_scaling_configuration_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DefaultAutoScalingConfigurationVersionArgs,
    ) -> DefaultAutoScalingConfigurationVersionResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DefaultAutoScalingConfigurationVersionArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> DefaultAutoScalingConfigurationVersionResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DefaultAutoScalingConfigurationVersionArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> DefaultAutoScalingConfigurationVersionResult {
        let auto_scaling_configuration_arn_binding = args
            .auto_scaling_configuration_arn
            .get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:apprunner/defaultAutoScalingConfigurationVersion:DefaultAutoScalingConfigurationVersion"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "autoScalingConfigurationArn".into(),
                    value: &auto_scaling_configuration_arn_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        DefaultAutoScalingConfigurationVersionResult {
            id: o.get_id(),
            urn: o.get_urn(),
            auto_scaling_configuration_arn: o.get_field("autoScalingConfigurationArn"),
        }
    }
}
