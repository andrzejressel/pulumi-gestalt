/// Manages status (recording / stopped) of an AWS Config Configuration Recorder.
///
/// > **Note:** Starting Configuration Recorder requires a Delivery Channel to be present. Use of `depends_on` (as shown below) is recommended to avoid race conditions.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   foo:
///     type: aws:cfg:RecorderStatus
///     properties:
///       name: ${fooRecorder.name}
///       isEnabled: true
///     options:
///       dependsOn:
///         - ${fooDeliveryChannel}
///   a:
///     type: aws:iam:RolePolicyAttachment
///     properties:
///       role: ${r.name}
///       policyArn: arn:aws:iam::aws:policy/service-role/AWS_ConfigRole
///   b:
///     type: aws:s3:BucketV2
///     properties:
///       bucket: awsconfig-example
///   fooDeliveryChannel:
///     type: aws:cfg:DeliveryChannel
///     name: foo
///     properties:
///       name: example
///       s3BucketName: ${b.bucket}
///   fooRecorder:
///     type: aws:cfg:Recorder
///     name: foo
///     properties:
///       name: example
///       roleArn: ${r.arn}
///   r:
///     type: aws:iam:Role
///     properties:
///       name: example-awsconfig
///       assumeRolePolicy: ${assumeRole.json}
///   pRolePolicy:
///     type: aws:iam:RolePolicy
///     name: p
///     properties:
///       name: awsconfig-example
///       role: ${r.id}
///       policy: ${p.json}
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
///   p:
///     fn::invoke:
///       function: aws:iam:getPolicyDocument
///       arguments:
///         statements:
///           - effect: Allow
///             actions:
///               - s3:*
///             resources:
///               - ${b.arn}
///               - ${b.arn}/*
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Configuration Recorder Status using the name of the Configuration Recorder. For example:
///
/// ```sh
/// $ pulumi import aws:cfg/recorderStatus:RecorderStatus foo example
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod recorder_status {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RecorderStatusArgs {
        /// Whether the configuration recorder should be enabled or disabled.
        #[builder(into)]
        pub is_enabled: pulumi_gestalt_rust::Input<bool>,
        /// The name of the recorder
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct RecorderStatusResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Whether the configuration recorder should be enabled or disabled.
        pub is_enabled: pulumi_gestalt_rust::Output<bool>,
        /// The name of the recorder
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RecorderStatusArgs,
    ) -> RecorderStatusResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RecorderStatusArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> RecorderStatusResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RecorderStatusArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> RecorderStatusResult {
        let is_enabled_binding = args.is_enabled.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cfg/recorderStatus:RecorderStatus".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "isEnabled".into(),
                    value: &is_enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        RecorderStatusResult {
            id: o.get_id(),
            urn: o.get_urn(),
            is_enabled: o.get_field("isEnabled"),
            name: o.get_field("name"),
        }
    }
}
