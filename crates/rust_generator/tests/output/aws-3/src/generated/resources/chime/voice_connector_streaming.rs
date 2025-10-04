/// Adds a streaming configuration for the specified Amazon Chime Voice Connector. The streaming configuration specifies whether media streaming is enabled for sending to Amazon Kinesis.
/// It also sets the retention period, in hours, for the Amazon Kinesis data.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let default = voice_connector::create(
///         "default",
///         VoiceConnectorArgs::builder()
///             .name("vc-name-test")
///             .require_encryption(true)
///             .build_struct(),
///     );
///     let defaultVoiceConnectorStreaming = voice_connector_streaming::create(
///         "defaultVoiceConnectorStreaming",
///         VoiceConnectorStreamingArgs::builder()
///             .data_retention(7)
///             .disabled(false)
///             .streaming_notification_targets(vec!["SQS",])
///             .voice_connector_id("${default.id}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Example Usage With Media Insights
///
/// ```yaml
/// resources:
///   default:
///     type: aws:chime:VoiceConnector
///     properties:
///       name: vc-name-test
///       requireEncryption: true
///   defaultVoiceConnectorStreaming:
///     type: aws:chime:VoiceConnectorStreaming
///     name: default
///     properties:
///       disabled: false
///       voiceConnectorId: ${default.id}
///       dataRetention: 7
///       streamingNotificationTargets:
///         - SQS
///       mediaInsightsConfiguration:
///         disabled: false
///         configurationArn: ${example.arn}
///   example:
///     type: aws:chimesdkmediapipelines:MediaInsightsPipelineConfiguration
///     properties:
///       name: ExampleConfig
///       resourceAccessRoleArn: ${exampleRole.arn}
///       elements:
///         - type: AmazonTranscribeCallAnalyticsProcessor
///           amazonTranscribeCallAnalyticsProcessorConfiguration:
///             languageCode: en-US
///         - type: KinesisDataStreamSink
///           kinesisDataStreamSinkConfiguration:
///             insightsTarget: ${exampleStream.arn}
///   exampleRole:
///     type: aws:iam:Role
///     name: example
///     properties:
///       name: ExampleResourceAccessRole
///       assumeRolePolicy: ${assumeRole.json}
///   exampleStream:
///     type: aws:kinesis:Stream
///     name: example
///     properties:
///       name: ExampleStream
///       shardCount: 2
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
///                   - mediapipelines.chime.amazonaws.com
///             actions:
///               - sts:AssumeRole
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Chime Voice Connector Streaming using the `voice_connector_id`. For example:
///
/// ```sh
/// $ pulumi import aws:chime/voiceConnectorStreaming:VoiceConnectorStreaming default abcdef1ghij2klmno3pqr4
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod voice_connector_streaming {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VoiceConnectorStreamingArgs {
        /// The retention period, in hours, for the Amazon Kinesis data.
        #[builder(into)]
        pub data_retention: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// When true, media streaming to Amazon Kinesis is turned off. Default: `false`
        #[builder(into, default)]
        pub disabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The media insights configuration. See `media_insights_configuration`.
        #[builder(into, default)]
        pub media_insights_configuration: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::chime::VoiceConnectorStreamingMediaInsightsConfiguration,
            >,
        >,
        /// The streaming notification targets. Valid Values: `EventBridge | SNS | SQS`
        #[builder(into, default)]
        pub streaming_notification_targets: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<String>>,
        >,
        /// The Amazon Chime Voice Connector ID.
        #[builder(into)]
        pub voice_connector_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VoiceConnectorStreamingResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The retention period, in hours, for the Amazon Kinesis data.
        pub data_retention: pulumi_gestalt_rust::Output<i32>,
        /// When true, media streaming to Amazon Kinesis is turned off. Default: `false`
        pub disabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The media insights configuration. See `media_insights_configuration`.
        pub media_insights_configuration: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::chime::VoiceConnectorStreamingMediaInsightsConfiguration,
            >,
        >,
        /// The streaming notification targets. Valid Values: `EventBridge | SNS | SQS`
        pub streaming_notification_targets: pulumi_gestalt_rust::Output<
            Option<Vec<String>>,
        >,
        /// The Amazon Chime Voice Connector ID.
        pub voice_connector_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VoiceConnectorStreamingArgs,
    ) -> VoiceConnectorStreamingResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let data_retention_binding = args.data_retention.get_output(context);
        let disabled_binding = args.disabled.get_output(context);
        let media_insights_configuration_binding = args
            .media_insights_configuration
            .get_output(context);
        let streaming_notification_targets_binding = args
            .streaming_notification_targets
            .get_output(context);
        let voice_connector_id_binding = args.voice_connector_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:chime/voiceConnectorStreaming:VoiceConnectorStreaming".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataRetention".into(),
                    value: &data_retention_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "disabled".into(),
                    value: &disabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mediaInsightsConfiguration".into(),
                    value: &media_insights_configuration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "streamingNotificationTargets".into(),
                    value: &streaming_notification_targets_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "voiceConnectorId".into(),
                    value: &voice_connector_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        VoiceConnectorStreamingResult {
            id: o.get_field("id"),
            data_retention: o.get_field("dataRetention"),
            disabled: o.get_field("disabled"),
            media_insights_configuration: o.get_field("mediaInsightsConfiguration"),
            streaming_notification_targets: o.get_field("streamingNotificationTargets"),
            voice_connector_id: o.get_field("voiceConnectorId"),
        }
    }
}
