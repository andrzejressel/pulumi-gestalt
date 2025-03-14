/// Logs-based metric can also be used to extract values from logs and create a a distribution
/// of the values. The distribution records the statistics of the extracted values along with
/// an optional histogram of the values as specified by the bucket options.
///
///
/// To get more information about Metric, see:
///
/// * [API documentation](https://cloud.google.com/logging/docs/reference/v2/rest/v2/projects.metrics/create)
/// * How-to Guides
///     * [Official Documentation](https://cloud.google.com/logging/docs/apis)
///
/// ## Example Usage
///
/// ### Logging Metric Basic
///
///
/// ```yaml
/// resources:
///   loggingMetric:
///     type: gcp:logging:Metric
///     name: logging_metric
///     properties:
///       name: my-(custom)/metric
///       filter: resource.type=gae_app AND severity>=ERROR
///       metricDescriptor:
///         metricKind: DELTA
///         valueType: DISTRIBUTION
///         unit: '1'
///         labels:
///           - key: mass
///             valueType: STRING
///             description: amount of matter
///           - key: sku
///             valueType: INT64
///             description: Identifying number for item
///         displayName: My metric
///       valueExtractor: EXTRACT(jsonPayload.request)
///       labelExtractors:
///         mass: EXTRACT(jsonPayload.request)
///         sku: EXTRACT(jsonPayload.id)
///       bucketOptions:
///         linearBuckets:
///           numFiniteBuckets: 3
///           width: 1
///           offset: 1
/// ```
/// ### Logging Metric Counter Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let loggingMetric = metric::create(
///         "loggingMetric",
///         MetricArgs::builder()
///             .filter("resource.type=gae_app AND severity>=ERROR")
///             .metric_descriptor(
///                 MetricMetricDescriptor::builder()
///                     .metricKind("DELTA")
///                     .valueType("INT64")
///                     .build_struct(),
///             )
///             .name("my-(custom)/metric")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Logging Metric Counter Labels
///
///
/// ```yaml
/// resources:
///   loggingMetric:
///     type: gcp:logging:Metric
///     name: logging_metric
///     properties:
///       name: my-(custom)/metric
///       filter: resource.type=gae_app AND severity>=ERROR
///       metricDescriptor:
///         metricKind: DELTA
///         valueType: INT64
///         labels:
///           - key: mass
///             valueType: STRING
///             description: amount of matter
///       labelExtractors:
///         mass: EXTRACT(jsonPayload.request)
/// ```
/// ### Logging Metric Logging Bucket
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let loggingMetric = project_bucket_config::create(
///         "loggingMetric",
///         ProjectBucketConfigArgs::builder()
///             .bucket_id("_Default")
///             .location("global")
///             .project("my-project-name")
///             .build_struct(),
///     );
///     let loggingMetricMetric = metric::create(
///         "loggingMetricMetric",
///         MetricArgs::builder()
///             .bucket_name("${loggingMetric.id}")
///             .filter("resource.type=gae_app AND severity>=ERROR")
///             .name("my-(custom)/metric")
///             .build_struct(),
///     );
/// }
/// ```
/// ### Logging Metric Disabled
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let loggingMetric = metric::create(
///         "loggingMetric",
///         MetricArgs::builder()
///             .disabled(true)
///             .filter("resource.type=gae_app AND severity>=ERROR")
///             .metric_descriptor(
///                 MetricMetricDescriptor::builder()
///                     .metricKind("DELTA")
///                     .valueType("INT64")
///                     .build_struct(),
///             )
///             .name("my-(custom)/metric")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Metric can be imported using any of these accepted formats:
///
/// * `{{project}} {{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Metric can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:logging/metric:Metric default "{{project}} {{name}}"
/// ```
///
/// ```sh
/// $ pulumi import gcp:logging/metric:Metric default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod metric {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MetricArgs {
        /// The resource name of the Log Bucket that owns the Log Metric. Only Log Buckets in projects
        /// are supported. The bucket has to be in the same project as the metric.
        #[builder(into, default)]
        pub bucket_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The bucketOptions are required when the logs-based metric is using a DISTRIBUTION value type and it
        /// describes the bucket boundaries used to create a histogram of the extracted values.
        /// Structure is documented below.
        #[builder(into, default)]
        pub bucket_options: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::logging::MetricBucketOptions>,
        >,
        /// A description of this metric, which is used in documentation. The maximum length of the
        /// description is 8000 characters.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// If set to True, then this metric is disabled and it does not generate any points.
        #[builder(into, default)]
        pub disabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// An advanced logs filter (https://cloud.google.com/logging/docs/view/advanced-filters) which
        /// is used to match log entries.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub filter: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A map from a label key string to an extractor expression which is used to extract data from a log
        /// entry field and assign as the label value. Each label key specified in the LabelDescriptor must
        /// have an associated extractor expression in this map. The syntax of the extractor expression is
        /// the same as for the valueExtractor field.
        #[builder(into, default)]
        pub label_extractors: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The optional metric descriptor associated with the logs-based metric.
        /// If unspecified, it uses a default metric descriptor with a DELTA metric kind,
        /// INT64 value type, with no labels and a unit of "1". Such a metric counts the
        /// number of log entries matching the filter expression.
        /// Structure is documented below.
        #[builder(into, default)]
        pub metric_descriptor: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::logging::MetricMetricDescriptor>,
        >,
        /// The client-assigned metric identifier. Examples - "error_count", "nginx/requests".
        /// Metric identifiers are limited to 100 characters and can include only the following
        /// characters A-Z, a-z, 0-9, and the special characters _-.,+!*',()%/. The forward-slash
        /// character (/) denotes a hierarchy of name pieces, and it cannot be the first character
        /// of the name.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A valueExtractor is required when using a distribution logs-based metric to extract the values to
        /// record from a log entry. Two functions are supported for value extraction - EXTRACT(field) or
        /// REGEXP_EXTRACT(field, regex). The argument are 1. field - The name of the log entry field from which
        /// the value is to be extracted. 2. regex - A regular expression using the Google RE2 syntax
        /// (https://github.com/google/re2/wiki/Syntax) with a single capture group to extract data from the specified
        /// log entry field. The value of the field is converted to a string before applying the regex. It is an
        /// error to specify a regex that does not include exactly one capture group.
        #[builder(into, default)]
        pub value_extractor: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct MetricResult {
        /// The resource name of the Log Bucket that owns the Log Metric. Only Log Buckets in projects
        /// are supported. The bucket has to be in the same project as the metric.
        pub bucket_name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The bucketOptions are required when the logs-based metric is using a DISTRIBUTION value type and it
        /// describes the bucket boundaries used to create a histogram of the extracted values.
        /// Structure is documented below.
        pub bucket_options: pulumi_gestalt_rust::Output<
            Option<super::super::types::logging::MetricBucketOptions>,
        >,
        /// A description of this metric, which is used in documentation. The maximum length of the
        /// description is 8000 characters.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// If set to True, then this metric is disabled and it does not generate any points.
        pub disabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// An advanced logs filter (https://cloud.google.com/logging/docs/view/advanced-filters) which
        /// is used to match log entries.
        ///
        ///
        /// - - -
        pub filter: pulumi_gestalt_rust::Output<String>,
        /// A map from a label key string to an extractor expression which is used to extract data from a log
        /// entry field and assign as the label value. Each label key specified in the LabelDescriptor must
        /// have an associated extractor expression in this map. The syntax of the extractor expression is
        /// the same as for the valueExtractor field.
        pub label_extractors: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// The optional metric descriptor associated with the logs-based metric.
        /// If unspecified, it uses a default metric descriptor with a DELTA metric kind,
        /// INT64 value type, with no labels and a unit of "1". Such a metric counts the
        /// number of log entries matching the filter expression.
        /// Structure is documented below.
        pub metric_descriptor: pulumi_gestalt_rust::Output<
            super::super::types::logging::MetricMetricDescriptor,
        >,
        /// The client-assigned metric identifier. Examples - "error_count", "nginx/requests".
        /// Metric identifiers are limited to 100 characters and can include only the following
        /// characters A-Z, a-z, 0-9, and the special characters _-.,+!*',()%/. The forward-slash
        /// character (/) denotes a hierarchy of name pieces, and it cannot be the first character
        /// of the name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// A valueExtractor is required when using a distribution logs-based metric to extract the values to
        /// record from a log entry. Two functions are supported for value extraction - EXTRACT(field) or
        /// REGEXP_EXTRACT(field, regex). The argument are 1. field - The name of the log entry field from which
        /// the value is to be extracted. 2. regex - A regular expression using the Google RE2 syntax
        /// (https://github.com/google/re2/wiki/Syntax) with a single capture group to extract data from the specified
        /// log entry field. The value of the field is converted to a string before applying the regex. It is an
        /// error to specify a regex that does not include exactly one capture group.
        pub value_extractor: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MetricArgs,
    ) -> MetricResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bucket_name_binding = args.bucket_name.get_output(context);
        let bucket_options_binding = args.bucket_options.get_output(context);
        let description_binding = args.description.get_output(context);
        let disabled_binding = args.disabled.get_output(context);
        let filter_binding = args.filter.get_output(context);
        let label_extractors_binding = args.label_extractors.get_output(context);
        let metric_descriptor_binding = args.metric_descriptor.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let value_extractor_binding = args.value_extractor.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:logging/metric:Metric".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucketName".into(),
                    value: &bucket_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bucketOptions".into(),
                    value: &bucket_options_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "disabled".into(),
                    value: &disabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filter".into(),
                    value: &filter_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "labelExtractors".into(),
                    value: &label_extractors_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metricDescriptor".into(),
                    value: &metric_descriptor_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "valueExtractor".into(),
                    value: &value_extractor_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        MetricResult {
            bucket_name: o.get_field("bucketName"),
            bucket_options: o.get_field("bucketOptions"),
            description: o.get_field("description"),
            disabled: o.get_field("disabled"),
            filter: o.get_field("filter"),
            label_extractors: o.get_field("labelExtractors"),
            metric_descriptor: o.get_field("metricDescriptor"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            value_extractor: o.get_field("valueExtractor"),
        }
    }
}
