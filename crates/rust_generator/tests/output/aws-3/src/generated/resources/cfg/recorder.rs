/// Provides an AWS Config Configuration Recorder. Please note that this resource **does not start** the created recorder automatically.
///
/// > **Note:** _Starting_ the Configuration Recorder requires a delivery channel (while delivery channel creation requires Configuration Recorder). This is why `aws.cfg.RecorderStatus` is a separate resource.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```yaml
/// resources:
///   foo:
///     type: aws:cfg:Recorder
///     properties:
///       name: example
///       roleArn: ${r.arn}
///   r:
///     type: aws:iam:Role
///     properties:
///       name: awsconfig-example
///       assumeRolePolicy: ${assumeRole.json}
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
///                   - config.amazonaws.com
///             actions:
///               - sts:AssumeRole
/// ```
///
/// ### Exclude Resources Types Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let foo = recorder::create(
///         "foo",
///         RecorderArgs::builder()
///             .name("example")
///             .recording_group(
///                 RecorderRecordingGroup::builder()
///                     .allSupported(false)
///                     .exclusionByResourceTypes(
///                         vec![
///                             RecorderRecordingGroupExclusionByResourceType::builder()
///                             .resourceTypes(vec!["AWS::EC2::Instance",]).build_struct(),
///                         ],
///                     )
///                     .recordingStrategies(
///                         vec![
///                             RecorderRecordingGroupRecordingStrategy::builder()
///                             .useOnly("EXCLUSION_BY_RESOURCE_TYPES").build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .role_arn("${r.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Periodic Recording
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let foo = recorder::create(
///         "foo",
///         RecorderArgs::builder()
///             .name("example")
///             .recording_group(
///                 RecorderRecordingGroup::builder()
///                     .allSupported(false)
///                     .includeGlobalResourceTypes(false)
///                     .resourceTypes(
///                         vec!["AWS::EC2::Instance", "AWS::EC2::NetworkInterface",],
///                     )
///                     .build_struct(),
///             )
///             .recording_mode(
///                 RecorderRecordingMode::builder()
///                     .recordingFrequency("CONTINUOUS")
///                     .recordingModeOverride(
///                         RecorderRecordingModeRecordingModeOverride::builder()
///                             .description("Only record EC2 network interfaces daily")
///                             .recordingFrequency("DAILY")
///                             .resourceTypes(vec!["AWS::EC2::NetworkInterface",])
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .role_arn("${r.arn}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Configuration Recorder using the name. For example:
///
/// ```sh
/// $ pulumi import aws:cfg/recorder:Recorder foo example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod recorder {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RecorderArgs {
        /// The name of the recorder. Defaults to `default`. Changing it recreates the resource.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Recording group - see below.
        #[builder(into, default)]
        pub recording_group: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cfg::RecorderRecordingGroup>,
        >,
        /// Recording mode - see below.
        #[builder(into, default)]
        pub recording_mode: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cfg::RecorderRecordingMode>,
        >,
        /// Amazon Resource Name (ARN) of the IAM role. Used to make read or write requests to the delivery channel and to describe the AWS resources associated with the account. See [AWS Docs](http://docs.aws.amazon.com/config/latest/developerguide/iamrole-permissions.html) for more details.
        #[builder(into)]
        pub role_arn: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RecorderResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The name of the recorder. Defaults to `default`. Changing it recreates the resource.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Recording group - see below.
        pub recording_group: pulumi_gestalt_rust::Output<
            super::super::types::cfg::RecorderRecordingGroup,
        >,
        /// Recording mode - see below.
        pub recording_mode: pulumi_gestalt_rust::Output<
            super::super::types::cfg::RecorderRecordingMode,
        >,
        /// Amazon Resource Name (ARN) of the IAM role. Used to make read or write requests to the delivery channel and to describe the AWS resources associated with the account. See [AWS Docs](http://docs.aws.amazon.com/config/latest/developerguide/iamrole-permissions.html) for more details.
        pub role_arn: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RecorderArgs,
    ) -> RecorderResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let recording_group_binding = args.recording_group.get_output(context);
        let recording_mode_binding = args.recording_mode.get_output(context);
        let role_arn_binding = args.role_arn.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cfg/recorder:Recorder".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "recordingGroup".into(),
                    value: &recording_group_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "recordingMode".into(),
                    value: &recording_mode_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "roleArn".into(),
                    value: &role_arn_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        RecorderResult {
            id: o.get_field("id"),
            name: o.get_field("name"),
            recording_group: o.get_field("recordingGroup"),
            recording_mode: o.get_field("recordingMode"),
            role_arn: o.get_field("roleArn"),
        }
    }
}
