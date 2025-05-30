/// Resource for managing an AWS Rekognition Stream Processor.
///
/// > This resource must be configured specifically for your use case, and not all options are compatible with one another. See [Stream Processor API documentation](https://docs.aws.amazon.com/rekognition/latest/APIReference/API_CreateStreamProcessor.html#rekognition-CreateStreamProcessor-request-Input) for configuration information.
///
/// > Stream Processors configured for Face Recognition cannot have _any_ properties updated after the fact, and it will result in an AWS API error.
///
/// ## Example Usage
///
/// ### Label Detection
///
/// ```yaml
/// resources:
///   example:
///     type: aws:s3:BucketV2
///     properties:
///       bucket: example-bucket
///   exampleTopic:
///     type: aws:sns:Topic
///     name: example
///     properties:
///       name: example-topic
///   exampleVideoStream:
///     type: aws:kinesis:VideoStream
///     name: example
///     properties:
///       name: example-kinesis-input
///       dataRetentionInHours: 1
///       deviceName: kinesis-video-device-name
///       mediaType: video/h264
///   exampleRole:
///     type: aws:iam:Role
///     name: example
///     properties:
///       name: example-role
///       inlinePolicies:
///         - name: Rekognition-Access
///           policy:
///             fn::toJSON:
///               Version: 2012-10-17
///               Statement:
///                 - Action:
///                     - s3:PutObject
///                   Effect: Allow
///                   Resource:
///                     - ${example.arn}/*
///                 - Action:
///                     - sns:Publish
///                   Effect: Allow
///                   Resource:
///                     - ${exampleTopic.arn}
///                 - Action:
///                     - kinesis:Get*
///                     - kinesis:DescribeStreamSummary
///                   Effect: Allow
///                   Resource:
///                     - ${exampleVideoStream.arn}
///       assumeRolePolicy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Action: sts:AssumeRole
///               Effect: Allow
///               Principal:
///                 Service: rekognition.amazonaws.com
///   exampleStreamProcessor:
///     type: aws:rekognition:StreamProcessor
///     name: example
///     properties:
///       roleArn: ${exampleRole.arn}
///       name: example-processor
///       dataSharingPreference:
///         optIn: false
///       output:
///         s3Destination:
///           bucket: ${example.bucket}
///       settings:
///         connectedHome:
///           labels:
///             - PERSON
///             - PET
///       input:
///         kinesisVideoStream:
///           arn: ${exampleVideoStream.arn}
///       notificationChannel:
///         snsTopicArn: ${exampleTopic.arn}
/// ```
///
/// ### Face Detection Usage
///
/// ```yaml
/// resources:
///   example:
///     type: aws:kinesis:VideoStream
///     properties:
///       name: example-kinesis-input
///       dataRetentionInHours: 1
///       deviceName: kinesis-video-device-name
///       mediaType: video/h264
///   exampleStream:
///     type: aws:kinesis:Stream
///     name: example
///     properties:
///       name: pulumi-kinesis-example
///       shardCount: 1
///   exampleRole:
///     type: aws:iam:Role
///     name: example
///     properties:
///       name: example-role
///       inlinePolicies:
///         - name: Rekognition-Access
///           policy:
///             fn::toJSON:
///               Version: 2012-10-17
///               Statement:
///                 - Action:
///                     - kinesis:Get*
///                     - kinesis:DescribeStreamSummary
///                   Effect: Allow
///                   Resource:
///                     - ${example.arn}
///                 - Action:
///                     - kinesis:PutRecord
///                   Effect: Allow
///                   Resource:
///                     - ${exampleStream.arn}
///       assumeRolePolicy:
///         fn::toJSON:
///           Version: 2012-10-17
///           Statement:
///             - Action: sts:AssumeRole
///               Effect: Allow
///               Principal:
///                 Service: rekognition.amazonaws.com
///   exampleCollection:
///     type: aws:rekognition:Collection
///     name: example
///     properties:
///       collectionId: example-collection
///   exampleStreamProcessor:
///     type: aws:rekognition:StreamProcessor
///     name: example
///     properties:
///       roleArn: ${exampleRole.arn}
///       name: example-processor
///       dataSharingPreference:
///         optIn: false
///       regionsOfInterests:
///         - polygons:
///             - x: 0.5
///               y: 0.5
///             - x: 0.5
///               y: 0.5
///             - x: 0.5
///               y: 0.5
///       input:
///         kinesisVideoStream:
///           arn: ${example.arn}
///       output:
///         kinesisDataStream:
///           arn: ${exampleStream.arn}
///       settings:
///         faceSearch:
///           collectionId: ${exampleCollection.id}
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Rekognition Stream Processor using the `name`. For example:
///
/// ```sh
/// $ pulumi import aws:rekognition/streamProcessor:StreamProcessor example my-stream
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod stream_processor {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct StreamProcessorArgs {
        /// See `data_sharing_preference`.
        #[builder(into, default)]
        pub data_sharing_preference: pulumi_gestalt_rust::InputOrOutput<
            Option<
                super::super::types::rekognition::StreamProcessorDataSharingPreference,
            >,
        >,
        /// Input video stream. See `input`.
        #[builder(into, default)]
        pub input: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::rekognition::StreamProcessorInput>,
        >,
        /// Optional parameter for label detection stream processors.
        #[builder(into, default)]
        pub kms_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Stream Processor.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Amazon Simple Notification Service topic to which Amazon Rekognition publishes the completion status. See `notification_channel`.
        #[builder(into, default)]
        pub notification_channel: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::rekognition::StreamProcessorNotificationChannel>,
        >,
        /// Kinesis data stream stream or Amazon S3 bucket location to which Amazon Rekognition Video puts the analysis results. See `output`.
        #[builder(into, default)]
        pub output: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::rekognition::StreamProcessorOutput>,
        >,
        /// Specifies locations in the frames where Amazon Rekognition checks for objects or people. See `regions_of_interest`.
        #[builder(into, default)]
        pub regions_of_interests: pulumi_gestalt_rust::InputOrOutput<
            Option<
                Vec<super::super::types::rekognition::StreamProcessorRegionsOfInterest>,
            >,
        >,
        /// The Amazon Resource Number (ARN) of the IAM role that allows access to the stream processor. The IAM role provides Rekognition read permissions for a Kinesis stream. It also provides write permissions to an Amazon S3 bucket and Amazon Simple Notification Service topic for a label detection stream processor. This is required for both face search and label detection stream processors.
        #[builder(into)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Input parameters used in a streaming video analyzed by a stream processor. See `settings`.
        ///
        /// The following arguments are optional:
        #[builder(into, default)]
        pub settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::rekognition::StreamProcessorSettings>,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
        #[builder(into, default)]
        pub timeouts: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::rekognition::StreamProcessorTimeouts>,
        >,
    }
    #[allow(dead_code)]
    pub struct StreamProcessorResult {
        /// See `data_sharing_preference`.
        pub data_sharing_preference: pulumi_gestalt_rust::Output<
            Option<
                super::super::types::rekognition::StreamProcessorDataSharingPreference,
            >,
        >,
        /// Input video stream. See `input`.
        pub input: pulumi_gestalt_rust::Output<
            Option<super::super::types::rekognition::StreamProcessorInput>,
        >,
        /// Optional parameter for label detection stream processors.
        pub kms_key_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Stream Processor.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The Amazon Simple Notification Service topic to which Amazon Rekognition publishes the completion status. See `notification_channel`.
        pub notification_channel: pulumi_gestalt_rust::Output<
            Option<super::super::types::rekognition::StreamProcessorNotificationChannel>,
        >,
        /// Kinesis data stream stream or Amazon S3 bucket location to which Amazon Rekognition Video puts the analysis results. See `output`.
        pub output: pulumi_gestalt_rust::Output<
            Option<super::super::types::rekognition::StreamProcessorOutput>,
        >,
        /// Specifies locations in the frames where Amazon Rekognition checks for objects or people. See `regions_of_interest`.
        pub regions_of_interests: pulumi_gestalt_rust::Output<
            Option<
                Vec<super::super::types::rekognition::StreamProcessorRegionsOfInterest>,
            >,
        >,
        /// The Amazon Resource Number (ARN) of the IAM role that allows access to the stream processor. The IAM role provides Rekognition read permissions for a Kinesis stream. It also provides write permissions to an Amazon S3 bucket and Amazon Simple Notification Service topic for a label detection stream processor. This is required for both face search and label detection stream processors.
        pub role_arn: pulumi_gestalt_rust::Output<String>,
        /// Input parameters used in a streaming video analyzed by a stream processor. See `settings`.
        ///
        /// The following arguments are optional:
        pub settings: pulumi_gestalt_rust::Output<
            Option<super::super::types::rekognition::StreamProcessorSettings>,
        >,
        /// ARN of the Stream Processor.
        pub stream_processor_arn: pulumi_gestalt_rust::Output<String>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
        pub timeouts: pulumi_gestalt_rust::Output<
            Option<super::super::types::rekognition::StreamProcessorTimeouts>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: StreamProcessorArgs,
    ) -> StreamProcessorResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let data_sharing_preference_binding = args
            .data_sharing_preference
            .get_output(context);
        let input_binding = args.input.get_output(context);
        let kms_key_id_binding = args.kms_key_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let notification_channel_binding = args.notification_channel.get_output(context);
        let output_binding = args.output.get_output(context);
        let regions_of_interests_binding = args.regions_of_interests.get_output(context);
        let role_arn_binding = args.role_arn.get_output(context);
        let settings_binding = args.settings.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let timeouts_binding = args.timeouts.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:rekognition/streamProcessor:StreamProcessor".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "dataSharingPreference".into(),
                    value: &data_sharing_preference_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "input".into(),
                    value: &input_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsKeyId".into(),
                    value: &kms_key_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "notificationChannel".into(),
                    value: &notification_channel_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "output".into(),
                    value: &output_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "regionsOfInterests".into(),
                    value: &regions_of_interests_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "settings".into(),
                    value: &settings_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "timeouts".into(),
                    value: &timeouts_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        StreamProcessorResult {
            data_sharing_preference: o.get_field("dataSharingPreference"),
            input: o.get_field("input"),
            kms_key_id: o.get_field("kmsKeyId"),
            name: o.get_field("name"),
            notification_channel: o.get_field("notificationChannel"),
            output: o.get_field("output"),
            regions_of_interests: o.get_field("regionsOfInterests"),
            role_arn: o.get_field("roleArn"),
            settings: o.get_field("settings"),
            stream_processor_arn: o.get_field("streamProcessorArn"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
            timeouts: o.get_field("timeouts"),
        }
    }
}
