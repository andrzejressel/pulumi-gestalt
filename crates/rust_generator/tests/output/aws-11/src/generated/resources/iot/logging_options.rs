/// Provides a resource to manage [default logging options](https://docs.aws.amazon.com/iot/latest/developerguide/configure-logging.html#configure-logging-console).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = logging_options::create(
///         "example",
///         LoggingOptionsArgs::builder()
///             .default_log_level("WARN")
///             .role_arn("${exampleAwsIamRole.arn}")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod logging_options {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LoggingOptionsArgs {
        /// The default logging level. Valid Values: `"DEBUG"`, `"INFO"`, `"ERROR"`, `"WARN"`, `"DISABLED"`.
        #[builder(into)]
        pub default_log_level: pulumi_gestalt_rust::InputOrOutput<String>,
        /// If `true` all logs are disabled. The default is `false`.
        #[builder(into, default)]
        pub disable_all_logs: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The ARN of the role that allows IoT to write to Cloudwatch logs.
        #[builder(into)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct LoggingOptionsResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The default logging level. Valid Values: `"DEBUG"`, `"INFO"`, `"ERROR"`, `"WARN"`, `"DISABLED"`.
        pub default_log_level: pulumi_gestalt_rust::Output<String>,
        /// If `true` all logs are disabled. The default is `false`.
        pub disable_all_logs: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The ARN of the role that allows IoT to write to Cloudwatch logs.
        pub role_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LoggingOptionsArgs,
    ) -> LoggingOptionsResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LoggingOptionsArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> LoggingOptionsResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LoggingOptionsArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> LoggingOptionsResult {
        let default_log_level_binding = args.default_log_level.get_output(ctx);
        let disable_all_logs_binding = args.disable_all_logs.get_output(ctx);
        let role_arn_binding = args.role_arn.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:iot/loggingOptions:LoggingOptions".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "defaultLogLevel".into(),
                    value: &default_log_level_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "disableAllLogs".into(),
                    value: &disable_all_logs_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        LoggingOptionsResult {
            id: o.get_id(),
            urn: o.get_urn(),
            default_log_level: o.get_field("defaultLogLevel"),
            disable_all_logs: o.get_field("disableAllLogs"),
            role_arn: o.get_field("roleArn"),
        }
    }
}
