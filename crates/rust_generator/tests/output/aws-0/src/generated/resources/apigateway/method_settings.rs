/// Manages API Gateway Stage Method Settings. For example, CloudWatch logging and metrics.
///
/// > **NOTE:** We recommend using this resource in conjunction with the `aws.apigateway.Stage` resource instead of a stage managed by the `aws.apigateway.Deployment` resource optional `stage_name` argument. Stages managed by the `aws.apigateway.Deployment` resource are recreated on redeployment and this resource will require a second apply to recreate the method settings.
///
/// ## Example Usage
///
/// ### End-to-end
///
///
/// ### Off
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let pathSpecific = method_settings::create(
///         "pathSpecific",
///         MethodSettingsArgs::builder()
///             .method_path("path1/GET")
///             .rest_api("${example.id}")
///             .settings(
///                 MethodSettingsSettings::builder().loggingLevel("OFF").build_struct(),
///             )
///             .stage_name("${exampleAwsApiGatewayStage.stageName}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Errors Only
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let pathSpecific = method_settings::create(
///         "pathSpecific",
///         MethodSettingsArgs::builder()
///             .method_path("path1/GET")
///             .rest_api("${example.id}")
///             .settings(
///                 MethodSettingsSettings::builder()
///                     .dataTraceEnabled(false)
///                     .loggingLevel("ERROR")
///                     .metricsEnabled(true)
///                     .build_struct(),
///             )
///             .stage_name("${exampleAwsApiGatewayStage.stageName}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Errors and Info Logs
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let pathSpecific = method_settings::create(
///         "pathSpecific",
///         MethodSettingsArgs::builder()
///             .method_path("path1/GET")
///             .rest_api("${example.id}")
///             .settings(
///                 MethodSettingsSettings::builder()
///                     .dataTraceEnabled(false)
///                     .loggingLevel("INFO")
///                     .metricsEnabled(true)
///                     .build_struct(),
///             )
///             .stage_name("${exampleAwsApiGatewayStage.stageName}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Full Request and Response Logs
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let pathSpecific = method_settings::create(
///         "pathSpecific",
///         MethodSettingsArgs::builder()
///             .method_path("path1/GET")
///             .rest_api("${example.id}")
///             .settings(
///                 MethodSettingsSettings::builder()
///                     .dataTraceEnabled(true)
///                     .loggingLevel("INFO")
///                     .metricsEnabled(true)
///                     .build_struct(),
///             )
///             .stage_name("${exampleAwsApiGatewayStage.stageName}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_api_gateway_method_settings` using `REST-API-ID/STAGE-NAME/METHOD-PATH`. For example:
///
/// ```sh
/// $ pulumi import aws:apigateway/methodSettings:MethodSettings example 12345abcde/example/test/GET
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod method_settings {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MethodSettingsArgs {
        /// Method path defined as `{resource_path}/{http_method}` for an individual method override, or `*/*` for overriding all methods in the stage. Ensure to trim any leading forward slashes in the path (e.g., `trimprefix(aws_api_gateway_resource.example.path, "/")`).
        #[builder(into)]
        pub method_path: pulumi_gestalt_rust::Input<String>,
        /// ID of the REST API
        #[builder(into)]
        pub rest_api: pulumi_gestalt_rust::Input<String>,
        /// Settings block, see below.
        #[builder(into)]
        pub settings: pulumi_gestalt_rust::Input<
            super::super::types::apigateway::MethodSettingsSettings,
        >,
        /// Name of the stage
        #[builder(into)]
        pub stage_name: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct MethodSettingsResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Method path defined as `{resource_path}/{http_method}` for an individual method override, or `*/*` for overriding all methods in the stage. Ensure to trim any leading forward slashes in the path (e.g., `trimprefix(aws_api_gateway_resource.example.path, "/")`).
        pub method_path: pulumi_gestalt_rust::Output<String>,
        /// ID of the REST API
        pub rest_api: pulumi_gestalt_rust::Output<String>,
        /// Settings block, see below.
        pub settings: pulumi_gestalt_rust::Output<
            super::super::types::apigateway::MethodSettingsSettings,
        >,
        /// Name of the stage
        pub stage_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MethodSettingsArgs,
    ) -> MethodSettingsResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MethodSettingsArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> MethodSettingsResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MethodSettingsArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> MethodSettingsResult {
        let method_path_binding = args.method_path.get_output(ctx);
        let rest_api_binding = args.rest_api.get_output(ctx);
        let settings_binding = args.settings.get_output(ctx);
        let stage_name_binding = args.stage_name.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:apigateway/methodSettings:MethodSettings".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "methodPath".into(),
                    value: &method_path_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "restApi".into(),
                    value: &rest_api_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "settings".into(),
                    value: &settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "stageName".into(),
                    value: &stage_name_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        MethodSettingsResult {
            id: o.get_id(),
            urn: o.get_urn(),
            method_path: o.get_field("methodPath"),
            rest_api: o.get_field("restApi"),
            settings: o.get_field("settings"),
            stage_name: o.get_field("stageName"),
        }
    }
}
