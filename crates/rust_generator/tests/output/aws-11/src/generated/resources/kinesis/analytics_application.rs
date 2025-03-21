/// Provides a Kinesis Analytics Application resource. Kinesis Analytics is a managed service that
/// allows processing and analyzing streaming data using standard SQL.
///
/// For more details, see the [Amazon Kinesis Analytics Documentation](https://docs.aws.amazon.com/kinesisanalytics/latest/dev/what-is.html).
///
/// > **Note:** To manage Amazon Kinesis Data Analytics for Apache Flink applications, use the `aws.kinesisanalyticsv2.Application` resource.
///
/// ## Example Usage
///
/// ### Kinesis Stream Input
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let testApplication = analytics_application::create(
///         "testApplication",
///         AnalyticsApplicationArgs::builder()
///             .inputs(
///                 AnalyticsApplicationInputs::builder()
///                     .kinesisStream(
///                         AnalyticsApplicationInputsKinesisStream::builder()
///                             .resourceArn("${testStream.arn}")
///                             .roleArn("${test.arn}")
///                             .build_struct(),
///                     )
///                     .namePrefix("test_prefix")
///                     .parallelism(
///                         AnalyticsApplicationInputsParallelism::builder()
///                             .count(1)
///                             .build_struct(),
///                     )
///                     .schema(
///                         AnalyticsApplicationInputsSchema::builder()
///                             .recordColumns(
///                                 vec![
///                                     AnalyticsApplicationInputsSchemaRecordColumn::builder()
///                                     .mapping("$.test").name("test").sqlType("VARCHAR(8)")
///                                     .build_struct(),
///                                 ],
///                             )
///                             .recordEncoding("UTF-8")
///                             .recordFormat(
///                                 AnalyticsApplicationInputsSchemaRecordFormat::builder()
///                                     .mappingParameters(
///                                         AnalyticsApplicationInputsSchemaRecordFormatMappingParameters::builder()
///                                             .json(
///                                                 AnalyticsApplicationInputsSchemaRecordFormatMappingParametersJson::builder()
///                                                     .recordRowPath("$")
///                                                     .build_struct(),
///                                             )
///                                             .build_struct(),
///                                     )
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .name("kinesis-analytics-application-test")
///             .build_struct(),
///     );
///     let testStream = stream::create(
///         "testStream",
///         StreamArgs::builder().name("kinesis-test").shard_count(1).build_struct(),
///     );
/// }
/// ```
///
/// ### Starting An Application
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = log_group::create(
///         "example",
///         LogGroupArgs::builder().name("analytics").build_struct(),
///     );
///     let exampleFirehoseDeliveryStream = firehose_delivery_stream::create(
///         "exampleFirehoseDeliveryStream",
///         FirehoseDeliveryStreamArgs::builder()
///             .destination("extended_s3")
///             .extended_s_3_configuration(
///                 FirehoseDeliveryStreamExtendedS3Configuration::builder()
///                     .bucketArn("${exampleAwsS3Bucket.arn}")
///                     .roleArn("${exampleAwsIamRole.arn}")
///                     .build_struct(),
///             )
///             .name("example-kinesis-delivery-stream")
///             .build_struct(),
///     );
///     let exampleLogStream = log_stream::create(
///         "exampleLogStream",
///         LogStreamArgs::builder()
///             .log_group_name("${example.name}")
///             .name("example-kinesis-application")
///             .build_struct(),
///     );
///     let exampleStream = stream::create(
///         "exampleStream",
///         StreamArgs::builder()
///             .name("example-kinesis-stream")
///             .shard_count(1)
///             .build_struct(),
///     );
///     let test = analytics_application::create(
///         "test",
///         AnalyticsApplicationArgs::builder()
///             .cloudwatch_logging_options(
///                 AnalyticsApplicationCloudwatchLoggingOptions::builder()
///                     .logStreamArn("${exampleLogStream.arn}")
///                     .roleArn("${exampleAwsIamRole.arn}")
///                     .build_struct(),
///             )
///             .inputs(
///                 AnalyticsApplicationInputs::builder()
///                     .kinesisStream(
///                         AnalyticsApplicationInputsKinesisStream::builder()
///                             .resourceArn("${exampleStream.arn}")
///                             .roleArn("${exampleAwsIamRole.arn}")
///                             .build_struct(),
///                     )
///                     .namePrefix("example_prefix")
///                     .schema(
///                         AnalyticsApplicationInputsSchema::builder()
///                             .recordColumns(
///                                 vec![
///                                     AnalyticsApplicationInputsSchemaRecordColumn::builder()
///                                     .name("COLUMN_1").sqlType("INTEGER").build_struct(),
///                                 ],
///                             )
///                             .recordFormat(
///                                 AnalyticsApplicationInputsSchemaRecordFormat::builder()
///                                     .mappingParameters(
///                                         AnalyticsApplicationInputsSchemaRecordFormatMappingParameters::builder()
///                                             .csv(
///                                                 AnalyticsApplicationInputsSchemaRecordFormatMappingParametersCsv::builder()
///                                                     .recordColumnDelimiter(",")
///                                                     .recordRowDelimiter("|")
///                                                     .build_struct(),
///                                             )
///                                             .build_struct(),
///                                     )
///                                     .build_struct(),
///                             )
///                             .build_struct(),
///                     )
///                     .startingPositionConfigurations(
///                         vec![
///                             AnalyticsApplicationInputsStartingPositionConfiguration::builder()
///                             .startingPosition("NOW").build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .name("example-application")
///             .outputs(
///                 vec![
///                     AnalyticsApplicationOutput::builder()
///                     .kinesisFirehose(AnalyticsApplicationOutputKinesisFirehose::builder()
///                     .resourceArn("${exampleFirehoseDeliveryStream.arn}")
///                     .roleArn("${exampleAwsIamRole.arn}").build_struct()).name("OUTPUT_1")
///                     .schema(AnalyticsApplicationOutputSchema::builder()
///                     .recordFormatType("CSV").build_struct()).build_struct(),
///                 ],
///             )
///             .start_application(true)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Kinesis Analytics Application using ARN. For example:
///
/// ```sh
/// $ pulumi import aws:kinesis/analyticsApplication:AnalyticsApplication example arn:aws:kinesisanalytics:us-west-2:1234567890:application/example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod analytics_application {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AnalyticsApplicationArgs {
        /// The CloudWatch log stream options to monitor application errors.
        /// See CloudWatch Logging Options below for more details.
        #[builder(into, default)]
        pub cloudwatch_logging_options: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::kinesis::AnalyticsApplicationCloudwatchLoggingOptions,
            >,
        >,
        /// SQL Code to transform input data, and generate output.
        #[builder(into, default)]
        pub code: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Description of the application.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Input configuration of the application. See Inputs below for more details.
        #[builder(into, default)]
        pub inputs: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::kinesis::AnalyticsApplicationInputs>,
        >,
        /// Name of the Kinesis Analytics Application.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Output destination configuration of the application. See Outputs below for more details.
        #[builder(into, default)]
        pub outputs: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::kinesis::AnalyticsApplicationOutput>>,
        >,
        /// An S3 Reference Data Source for the application.
        /// See Reference Data Sources below for more details.
        #[builder(into, default)]
        pub reference_data_sources: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::kinesis::AnalyticsApplicationReferenceDataSources,
            >,
        >,
        /// Whether to start or stop the Kinesis Analytics Application. To start an application, an input with a defined `starting_position` must be configured.
        /// To modify an application's starting position, first stop the application by setting `start_application = false`, then update `starting_position` and set `start_application = true`.
        #[builder(into, default)]
        pub start_application: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Key-value map of tags for the Kinesis Analytics Application. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct AnalyticsApplicationResult {
        /// The ARN of the Kinesis Analytics Appliation.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The CloudWatch log stream options to monitor application errors.
        /// See CloudWatch Logging Options below for more details.
        pub cloudwatch_logging_options: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::kinesis::AnalyticsApplicationCloudwatchLoggingOptions,
            >,
        >,
        /// SQL Code to transform input data, and generate output.
        pub code: pulumi_gestalt_rust::Output<Option<String>>,
        /// The Timestamp when the application version was created.
        pub create_timestamp: pulumi_gestalt_rust::Output<String>,
        /// Description of the application.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// Input configuration of the application. See Inputs below for more details.
        pub inputs: pulumi_gestalt_rust::Output<
            Option<super::super::types::kinesis::AnalyticsApplicationInputs>,
        >,
        /// The Timestamp when the application was last updated.
        pub last_update_timestamp: pulumi_gestalt_rust::Output<String>,
        /// Name of the Kinesis Analytics Application.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Output destination configuration of the application. See Outputs below for more details.
        pub outputs: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::kinesis::AnalyticsApplicationOutput>>,
        >,
        /// An S3 Reference Data Source for the application.
        /// See Reference Data Sources below for more details.
        pub reference_data_sources: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::kinesis::AnalyticsApplicationReferenceDataSources,
            >,
        >,
        /// Whether to start or stop the Kinesis Analytics Application. To start an application, an input with a defined `starting_position` must be configured.
        /// To modify an application's starting position, first stop the application by setting `start_application = false`, then update `starting_position` and set `start_application = true`.
        pub start_application: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The Status of the application.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// Key-value map of tags for the Kinesis Analytics Application. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        /// The Version of the application.
        pub version: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AnalyticsApplicationArgs,
    ) -> AnalyticsApplicationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let cloudwatch_logging_options_binding = args
            .cloudwatch_logging_options
            .get_output(context);
        let code_binding = args.code.get_output(context);
        let description_binding = args.description.get_output(context);
        let inputs_binding = args.inputs.get_output(context);
        let name_binding = args.name.get_output(context);
        let outputs_binding = args.outputs.get_output(context);
        let reference_data_sources_binding = args
            .reference_data_sources
            .get_output(context);
        let start_application_binding = args.start_application.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:kinesis/analyticsApplication:AnalyticsApplication".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cloudwatchLoggingOptions".into(),
                    value: &cloudwatch_logging_options_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "code".into(),
                    value: &code_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "inputs".into(),
                    value: &inputs_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "outputs".into(),
                    value: &outputs_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "referenceDataSources".into(),
                    value: &reference_data_sources_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "startApplication".into(),
                    value: &start_application_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AnalyticsApplicationResult {
            arn: o.get_field("arn"),
            cloudwatch_logging_options: o.get_field("cloudwatchLoggingOptions"),
            code: o.get_field("code"),
            create_timestamp: o.get_field("createTimestamp"),
            description: o.get_field("description"),
            inputs: o.get_field("inputs"),
            last_update_timestamp: o.get_field("lastUpdateTimestamp"),
            name: o.get_field("name"),
            outputs: o.get_field("outputs"),
            reference_data_sources: o.get_field("referenceDataSources"),
            start_application: o.get_field("startApplication"),
            status: o.get_field("status"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            version: o.get_field("version"),
        }
    }
}
