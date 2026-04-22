/// Provides a resource to manage the AWS Config retention configuration.
/// The retention configuration defines the number of days that AWS Config stores historical information.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = retention_configuration::create(
///         "example",
///         RetentionConfigurationArgs::builder().retention_period_in_days(90).build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import the AWS Config retention configuration using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:cfg/retentionConfiguration:RetentionConfiguration example default
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod retention_configuration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RetentionConfigurationArgs {
        /// The number of days AWS Config stores historical information.
        #[builder(into)]
        pub retention_period_in_days: pulumi_gestalt_rust::Input<i32>,
    }
    #[allow(dead_code)]
    pub struct RetentionConfigurationResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The name of the retention configuration object. The object is always named **default**.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The number of days AWS Config stores historical information.
        pub retention_period_in_days: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RetentionConfigurationArgs,
    ) -> RetentionConfigurationResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RetentionConfigurationArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> RetentionConfigurationResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RetentionConfigurationArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> RetentionConfigurationResult {
        let retention_period_in_days_binding = args
            .retention_period_in_days
            .get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cfg/retentionConfiguration:RetentionConfiguration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "retentionPeriodInDays".into(),
                    value: &retention_period_in_days_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        RetentionConfigurationResult {
            id: o.get_id(),
            urn: o.get_urn(),
            name: o.get_field("name"),
            retention_period_in_days: o.get_field("retentionPeriodInDays"),
        }
    }
}
