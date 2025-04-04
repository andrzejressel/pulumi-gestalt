/// Provides a CloudFront real-time log configuration resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleRole:
///     type: aws:iam:Role
///     name: example
///     properties:
///       name: cloudfront-realtime-log-config-example
///       assumeRolePolicy: ${assumeRole.json}
///   exampleRolePolicy:
///     type: aws:iam:RolePolicy
///     name: example
///     properties:
///       name: cloudfront-realtime-log-config-example
///       role: ${exampleRole.id}
///       policy: ${example.json}
///   exampleRealtimeLogConfig:
///     type: aws:cloudfront:RealtimeLogConfig
///     name: example
///     properties:
///       name: example
///       samplingRate: 75
///       fields:
///         - timestamp
///         - c-ip
///       endpoint:
///         streamType: Kinesis
///         kinesisStreamConfig:
///           roleArn: ${exampleRole.arn}
///           streamArn: ${exampleAwsKinesisStream.arn}
///     options:
///       dependsOn:
///         - ${exampleRolePolicy}
/// variables:
///   assumeRole:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             principals:
///               - type: Service
///                 identifiers:
///                   - cloudfront.amazonaws.com
///             actions:
///               - sts:AssumeRole
///   example:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             actions:
///               - kinesis:DescribeStreamSummary
///               - kinesis:DescribeStream
///               - kinesis:PutRecord
///               - kinesis:PutRecords
///             resources:
///               - ${exampleAwsKinesisStream.arn}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CloudFront real-time log configurations using the ARN. For example:
///
/// ```sh
/// $ pulumi import aws:cloudfront/realtimeLogConfig:RealtimeLogConfig example arn:aws:cloudfront::111122223333:realtime-log-config/ExampleNameForRealtimeLogConfig
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod realtime_log_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RealtimeLogConfigArgs {
        /// The Amazon Kinesis data streams where real-time log data is sent.
        #[builder(into)]
        pub endpoint: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::cloudfront::RealtimeLogConfigEndpoint,
        >,
        /// The fields that are included in each real-time log record. See the [AWS documentation](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/real-time-logs.html#understand-real-time-log-config-fields) for supported values.
        #[builder(into)]
        pub fields: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The unique name to identify this real-time log configuration.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The sampling rate for this real-time log configuration. The sampling rate determines the percentage of viewer requests that are represented in the real-time log data. An integer between `1` and `100`, inclusive.
        #[builder(into)]
        pub sampling_rate: pulumi_gestalt_rust::InputOrOutput<i32>,
    }
    #[allow(dead_code)]
    pub struct RealtimeLogConfigResult {
        /// The ARN (Amazon Resource Name) of the CloudFront real-time log configuration.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Kinesis data streams where real-time log data is sent.
        pub endpoint: pulumi_gestalt_rust::Output<
            super::super::types::cloudfront::RealtimeLogConfigEndpoint,
        >,
        /// The fields that are included in each real-time log record. See the [AWS documentation](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/real-time-logs.html#understand-real-time-log-config-fields) for supported values.
        pub fields: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The unique name to identify this real-time log configuration.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The sampling rate for this real-time log configuration. The sampling rate determines the percentage of viewer requests that are represented in the real-time log data. An integer between `1` and `100`, inclusive.
        pub sampling_rate: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RealtimeLogConfigArgs,
    ) -> RealtimeLogConfigResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let endpoint_binding = args.endpoint.get_output(context);
        let fields_binding = args.fields.get_output(context);
        let name_binding = args.name.get_output(context);
        let sampling_rate_binding = args.sampling_rate.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cloudfront/realtimeLogConfig:RealtimeLogConfig".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "endpoint".into(),
                    value: &endpoint_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "fields".into(),
                    value: &fields_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "samplingRate".into(),
                    value: &sampling_rate_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        RealtimeLogConfigResult {
            arn: o.get_field("arn"),
            endpoint: o.get_field("endpoint"),
            fields: o.get_field("fields"),
            name: o.get_field("name"),
            sampling_rate: o.get_field("samplingRate"),
        }
    }
}
