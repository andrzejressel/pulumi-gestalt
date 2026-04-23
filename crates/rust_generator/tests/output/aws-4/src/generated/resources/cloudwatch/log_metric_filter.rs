/// Provides a CloudWatch Log Metric Filter resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let dada = log_group::create(
///         "dada",
///         LogGroupArgs::builder().name("MyApp/access.log").build_struct(),
///     );
///     let yada = log_metric_filter::create(
///         "yada",
///         LogMetricFilterArgs::builder()
///             .log_group_name("${dada.name}")
///             .metric_transformation(
///                 LogMetricFilterMetricTransformation::builder()
///                     .name("EventCount")
///                     .namespace("YourNamespace")
///                     .value("1")
///                     .build_struct(),
///             )
///             .name("MyAppAccessCount")
///             .pattern("")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CloudWatch Log Metric Filter using the `log_group_name:name`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudwatch/logMetricFilter:LogMetricFilter test /aws/lambda/function:test
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod log_metric_filter {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LogMetricFilterArgs {
        /// The name of the log group to associate the metric filter with.
        #[builder(into)]
        pub log_group_name: pulumi_gestalt_rust::Input<String>,
        /// A block defining collection of information needed to define how metric data gets emitted. See below.
        #[builder(into)]
        pub metric_transformation: pulumi_gestalt_rust::Input<
            super::super::types::cloudwatch::LogMetricFilterMetricTransformation,
        >,
        /// A name for the metric filter.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// A valid [CloudWatch Logs filter pattern](https://docs.aws.amazon.com/AmazonCloudWatch/latest/DeveloperGuide/FilterAndPatternSyntax.html)
        /// for extracting metric data out of ingested log events.
        #[builder(into)]
        pub pattern: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct LogMetricFilterResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The name of the log group to associate the metric filter with.
        pub log_group_name: pulumi_gestalt_rust::Output<String>,
        /// A block defining collection of information needed to define how metric data gets emitted. See below.
        pub metric_transformation: pulumi_gestalt_rust::Output<
            super::super::types::cloudwatch::LogMetricFilterMetricTransformation,
        >,
        /// A name for the metric filter.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A valid [CloudWatch Logs filter pattern](https://docs.aws.amazon.com/AmazonCloudWatch/latest/DeveloperGuide/FilterAndPatternSyntax.html)
        /// for extracting metric data out of ingested log events.
        pub pattern: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LogMetricFilterArgs,
    ) -> LogMetricFilterResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LogMetricFilterArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> LogMetricFilterResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LogMetricFilterArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> LogMetricFilterResult {
        let log_group_name_binding = args.log_group_name.get_output(ctx);
        let metric_transformation_binding = args.metric_transformation.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let pattern_binding = args.pattern.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cloudwatch/logMetricFilter:LogMetricFilter".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logGroupName".into(),
                    value: &log_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metricTransformation".into(),
                    value: &metric_transformation_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "pattern".into(),
                    value: &pattern_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        LogMetricFilterResult {
            id: o.get_id(),
            urn: o.get_urn(),
            log_group_name: o.get_field("logGroupName"),
            metric_transformation: o.get_field("metricTransformation"),
            name: o.get_field("name"),
            pattern: o.get_field("pattern"),
        }
    }
}
