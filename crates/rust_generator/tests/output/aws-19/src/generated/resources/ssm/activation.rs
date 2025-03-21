/// Registers an on-premises server or virtual machine with Amazon EC2 so that it can be managed using Run Command.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   testRole:
///     type: aws:iam:Role
///     name: test_role
///     properties:
///       name: test_role
///       assumeRolePolicy: ${assumeRole.json}
///   testAttach:
///     type: aws:iam:RolePolicyAttachment
///     name: test_attach
///     properties:
///       role: ${testRole.name}
///       policyArn: arn:aws:iam::aws:policy/AmazonSSMManagedInstanceCore
///   foo:
///     type: aws:ssm:Activation
///     properties:
///       name: test_ssm_activation
///       description: Test
///       iamRole: ${testRole.id}
///       registrationLimit: '5'
///     options:
///       dependsOn:
///         - ${testAttach}
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
///                   - ssm.amazonaws.com
///             actions:
///               - sts:AssumeRole
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import AWS SSM Activation using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:ssm/activation:Activation example e488f2f6-e686-4afb-8a04-ef6dfEXAMPLE
/// ```
/// -> __Note:__ The `activation_code` attribute cannot be imported.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod activation {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ActivationArgs {
        /// The description of the resource that you want to register.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// UTC timestamp in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8) by which this activation request should expire. The default value is 24 hours from resource creation time. This provider will only perform drift detection of its value when present in a configuration.
        #[builder(into, default)]
        pub expiration_date: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The IAM Role to attach to the managed instance.
        #[builder(into)]
        pub iam_role: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The default name of the registered managed instance.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The maximum number of managed instances you want to register. The default value is 1 instance.
        #[builder(into, default)]
        pub registration_limit: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// A map of tags to assign to the object. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct ActivationResult {
        /// The code the system generates when it processes the activation.
        pub activation_code: pulumi_gestalt_rust::Output<String>,
        /// The description of the resource that you want to register.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// UTC timestamp in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8) by which this activation request should expire. The default value is 24 hours from resource creation time. This provider will only perform drift detection of its value when present in a configuration.
        pub expiration_date: pulumi_gestalt_rust::Output<String>,
        /// If the current activation has expired.
        pub expired: pulumi_gestalt_rust::Output<bool>,
        /// The IAM Role to attach to the managed instance.
        pub iam_role: pulumi_gestalt_rust::Output<String>,
        /// The default name of the registered managed instance.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The number of managed instances that are currently registered using this activation.
        pub registration_count: pulumi_gestalt_rust::Output<i32>,
        /// The maximum number of managed instances you want to register. The default value is 1 instance.
        pub registration_limit: pulumi_gestalt_rust::Output<Option<i32>>,
        /// A map of tags to assign to the object. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
        /// A map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
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
        args: ActivationArgs,
    ) -> ActivationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let expiration_date_binding = args.expiration_date.get_output(context);
        let iam_role_binding = args.iam_role.get_output(context);
        let name_binding = args.name.get_output(context);
        let registration_limit_binding = args.registration_limit.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ssm/activation:Activation".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "expirationDate".into(),
                    value: &expiration_date_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "iamRole".into(),
                    value: &iam_role_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "registrationLimit".into(),
                    value: &registration_limit_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ActivationResult {
            activation_code: o.get_field("activationCode"),
            description: o.get_field("description"),
            expiration_date: o.get_field("expirationDate"),
            expired: o.get_field("expired"),
            iam_role: o.get_field("iamRole"),
            name: o.get_field("name"),
            registration_count: o.get_field("registrationCount"),
            registration_limit: o.get_field("registrationLimit"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
