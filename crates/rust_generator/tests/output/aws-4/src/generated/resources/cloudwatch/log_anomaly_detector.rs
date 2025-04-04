/// Resource for managing an AWS CloudWatch Logs Log Anomaly Detector.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   test:
///     type: aws:cloudwatch:LogGroup
///     properties:
///       name: testing-${range.value}
///     options: {}
///   testLogAnomalyDetector:
///     type: aws:cloudwatch:LogAnomalyDetector
///     name: test
///     properties:
///       detectorName: testing
///       logGroupArnLists:
///         - ${test[0].arn}
///       anomalyVisibilityTime: 7
///       evaluationFrequency: TEN_MIN
///       enabled: 'false'
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import CloudWatch Log Anomaly Detector using the `example_id_arg`. For example:
///
/// ```sh
/// $ pulumi import aws:cloudwatch/logAnomalyDetector:LogAnomalyDetector example log_anomaly_detector-arn-12345678
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod log_anomaly_detector {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LogAnomalyDetectorArgs {
        /// Number of days to have visibility on an anomaly. After this time period has elapsed for an anomaly, it will be automatically baselined and the anomaly detector will treat new occurrences of a similar anomaly as normal. Therefore, if you do not correct the cause of an anomaly during the time period specified in `anomaly_visibility_time`, it will be considered normal going forward and will not be detected as an anomaly. Valid Range: Minimum value of 7. Maximum value of 90.
        #[builder(into, default)]
        pub anomaly_visibility_time: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Name for this anomaly detector.
        #[builder(into, default)]
        pub detector_name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// Specifies how often the anomaly detector is to run and look for anomalies. Set this value according to the frequency that the log group receives new logs. For example, if the log group receives new log events every 10 minutes, then 15 minutes might be a good setting for `evaluation_frequency`. Valid Values: `ONE_MIN | FIVE_MIN | TEN_MIN | FIFTEEN_MIN | THIRTY_MIN | ONE_HOUR`.
        #[builder(into, default)]
        pub evaluation_frequency: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// You can use this parameter to limit the anomaly detection model to examine only log events that match the pattern you specify here. For more information, see [Filter and Pattern Syntax](https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/FilterAndPatternSyntax.html).
        #[builder(into, default)]
        pub filter_pattern: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Optionally assigns a AWS KMS key to secure this anomaly detector and its findings. If a key is assigned, the anomalies found and the model used by this detector are encrypted at rest with the key. If a key is assigned to an anomaly detector, a user must have permissions for both this key and for the anomaly detector to retrieve information about the anomalies that it finds.
        #[builder(into, default)]
        pub kms_key_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Array containing the ARN of the log group that this anomaly detector will watch. You can specify only one log group ARN.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub log_group_arn_lists: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct LogAnomalyDetectorResult {
        /// Number of days to have visibility on an anomaly. After this time period has elapsed for an anomaly, it will be automatically baselined and the anomaly detector will treat new occurrences of a similar anomaly as normal. Therefore, if you do not correct the cause of an anomaly during the time period specified in `anomaly_visibility_time`, it will be considered normal going forward and will not be detected as an anomaly. Valid Range: Minimum value of 7. Maximum value of 90.
        pub anomaly_visibility_time: pulumi_gestalt_rust::Output<i32>,
        /// ARN of the log anomaly detector that you just created.
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Name for this anomaly detector.
        pub detector_name: pulumi_gestalt_rust::Output<Option<String>>,
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        /// Specifies how often the anomaly detector is to run and look for anomalies. Set this value according to the frequency that the log group receives new logs. For example, if the log group receives new log events every 10 minutes, then 15 minutes might be a good setting for `evaluation_frequency`. Valid Values: `ONE_MIN | FIVE_MIN | TEN_MIN | FIFTEEN_MIN | THIRTY_MIN | ONE_HOUR`.
        pub evaluation_frequency: pulumi_gestalt_rust::Output<Option<String>>,
        /// You can use this parameter to limit the anomaly detection model to examine only log events that match the pattern you specify here. For more information, see [Filter and Pattern Syntax](https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/FilterAndPatternSyntax.html).
        pub filter_pattern: pulumi_gestalt_rust::Output<Option<String>>,
        /// Optionally assigns a AWS KMS key to secure this anomaly detector and its findings. If a key is assigned, the anomalies found and the model used by this detector are encrypted at rest with the key. If a key is assigned to an anomaly detector, a user must have permissions for both this key and for the anomaly detector to retrieve information about the anomalies that it finds.
        pub kms_key_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Array containing the ARN of the log group that this anomaly detector will watch. You can specify only one log group ARN.
        ///
        /// The following arguments are optional:
        pub log_group_arn_lists: pulumi_gestalt_rust::Output<Vec<String>>,
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        pub tags_all: pulumi_gestalt_rust::Output<
            std::collections::HashMap<String, String>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: LogAnomalyDetectorArgs,
    ) -> LogAnomalyDetectorResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let anomaly_visibility_time_binding = args
            .anomaly_visibility_time
            .get_output(context);
        let detector_name_binding = args.detector_name.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let evaluation_frequency_binding = args.evaluation_frequency.get_output(context);
        let filter_pattern_binding = args.filter_pattern.get_output(context);
        let kms_key_id_binding = args.kms_key_id.get_output(context);
        let log_group_arn_lists_binding = args.log_group_arn_lists.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cloudwatch/logAnomalyDetector:LogAnomalyDetector".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "anomalyVisibilityTime".into(),
                    value: &anomaly_visibility_time_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "detectorName".into(),
                    value: &detector_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "evaluationFrequency".into(),
                    value: &evaluation_frequency_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filterPattern".into(),
                    value: &filter_pattern_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsKeyId".into(),
                    value: &kms_key_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "logGroupArnLists".into(),
                    value: &log_group_arn_lists_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        LogAnomalyDetectorResult {
            anomaly_visibility_time: o.get_field("anomalyVisibilityTime"),
            arn: o.get_field("arn"),
            detector_name: o.get_field("detectorName"),
            enabled: o.get_field("enabled"),
            evaluation_frequency: o.get_field("evaluationFrequency"),
            filter_pattern: o.get_field("filterPattern"),
            kms_key_id: o.get_field("kmsKeyId"),
            log_group_arn_lists: o.get_field("logGroupArnLists"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
