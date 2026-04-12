/// Provides a CloudWatch Log Stream resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let foo = log_stream::create(
///         "foo",
///         LogStreamArgs::builder()
///             .log_group_name("${yada.name}")
///             .name("SampleLogStream1234")
///             .build_struct(),
///     );
///     let yada = log_group::create(
///         "yada",
///         LogGroupArgs::builder().name("Yada").build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Cloudwatch Log Stream using the stream's `log_group_name` and `name`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudwatch/logStream:LogStream foo Yada:SampleLogStream1234
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod log_stream {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LogStreamArgs {
        /// The name of the log group under which the log stream is to be created.
        #[builder(into)]
        pub log_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the log stream. Must not be longer than 512 characters and must not contain `:`
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct LogStreamResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Resource Name (ARN) specifying the log stream.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The name of the log group under which the log stream is to be created.
        pub log_group_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the log stream. Must not be longer than 512 characters and must not contain `:`
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LogStreamArgs,
    ) -> LogStreamResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LogStreamArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> LogStreamResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LogStreamArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> LogStreamResult {
        let log_group_name_binding = args.log_group_name.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cloudwatch/logStream:LogStream".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logGroupName".into(),
                    value: &log_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        LogStreamResult {
            id: o.get_id(),
            urn: o.get_urn(),
            arn: o.get_field("arn"),
            log_group_name: o.get_field("logGroupName"),
            name: o.get_field("name"),
        }
    }
}
