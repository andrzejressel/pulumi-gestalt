/// Provides an AWS Config Rule.
///
/// > **Note:** Config Rule requires an existing Configuration Recorder to be present. Use of `depends_on` is recommended (as shown below) to avoid race conditions.
///
/// ## Example Usage
///
/// ### AWS Managed Rules
///
/// AWS managed rules can be used by setting the source owner to `AWS` and the source identifier to the name of the managed rule. More information about AWS managed rules can be found in the [AWS Config Developer Guide](https://docs.aws.amazon.com/config/latest/developerguide/evaluate-config_use-managed-rules.html).
///
/// ```yaml
/// resources:
///   r:
///     type: aws:cfg:Rule
///     properties:
///       name: example
///       source:
///         owner: AWS
///         sourceIdentifier: S3_BUCKET_VERSIONING_ENABLED
///     options:
///       dependsOn:
///         - ${foo}
///   foo:
///     type: aws:cfg:Recorder
///     properties:
///       name: example
///       roleArn: ${rRole.arn}
///   rRole:
///     type: aws:iam:Role
///     name: r
///     properties:
///       name: my-awsconfig-role
///       assumeRolePolicy: ${assumeRole.json}
///   pRolePolicy:
///     type: aws:iam:RolePolicy
///     name: p
///     properties:
///       name: my-awsconfig-policy
///       role: ${rRole.id}
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
///               - config:Put*
///             resources:
///               - '*'
/// ```
///
/// ### Custom Rules
///
/// Custom rules can be used by setting the source owner to `CUSTOM_LAMBDA` and the source identifier to the Amazon Resource Name (ARN) of the Lambda Function. The AWS Config service must have permissions to invoke the Lambda Function, e.g., via the `aws.lambda.Permission` resource. More information about custom rules can be found in the [AWS Config Developer Guide](https://docs.aws.amazon.com/config/latest/developerguide/evaluate-config_develop-rules.html).
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = recorder::create("example", RecorderArgs::builder().build_struct());
///     let exampleFunction = function::create(
///         "exampleFunction",
///         FunctionArgs::builder().build_struct(),
///     );
///     let examplePermission = permission::create(
///         "examplePermission",
///         PermissionArgs::builder()
///             .action("lambda:InvokeFunction")
///             .function("${exampleFunction.arn}")
///             .principal("config.amazonaws.com")
///             .statement_id("AllowExecutionFromConfig")
///             .build_struct(),
///     );
///     let exampleRule = rule::create(
///         "exampleRule",
///         RuleArgs::builder()
///             .source(
///                 RuleSource::builder()
///                     .owner("CUSTOM_LAMBDA")
///                     .sourceIdentifier("${exampleFunction.arn}")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ### Custom Policies
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = rule::create(
///         "example",
///         RuleArgs::builder()
///             .name("example")
///             .source(
///                 RuleSource::builder()
///                     .customPolicyDetails(
///                         RuleSourceCustomPolicyDetails::builder()
///                             .policyRuntime("guard-2.x.x")
///                             .policyText(
///                                 "\t  rule tableisactive when\n\t\t  resourceType == \"AWS::DynamoDB::Table\" {\n\t\t  configuration.tableStatus == ['ACTIVE']\n\t  }\n\t  \n\t  rule checkcompliance when\n\t\t  resourceType == \"AWS::DynamoDB::Table\"\n\t\t  tableisactive {\n\t\t\t  supplementaryConfiguration.ContinuousBackupsDescription.pointInTimeRecoveryDescription.pointInTimeRecoveryStatus == \"ENABLED\"\n\t  }\n",
///                             )
///                             .build_struct(),
///                     )
///                     .owner("CUSTOM_POLICY")
///                     .sourceDetails(
///                         vec![
///                             RuleSourceSourceDetail::builder()
///                             .messageType("ConfigurationItemChangeNotification")
///                             .build_struct(),
///                         ],
///                     )
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Config Rule using the name. For example:
///
/// ```sh
/// $ pulumi import aws:cfg/rule:Rule foo example
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RuleArgs {
        /// Description of the rule
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The modes the Config rule can be evaluated in. See Evaluation Mode for more details.
        #[builder(into, default)]
        pub evaluation_modes: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::cfg::RuleEvaluationMode>>,
        >,
        /// A string in JSON format that is passed to the AWS Config rule Lambda function.
        #[builder(into, default)]
        pub input_parameters: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The maximum frequency with which AWS Config runs evaluations for a rule.
        #[builder(into, default)]
        pub maximum_execution_frequency: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// The name of the rule
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Scope defines which resources can trigger an evaluation for the rule. See Scope Below.
        #[builder(into, default)]
        pub scope: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cfg::RuleScope>,
        >,
        /// Source specifies the rule owner, the rule identifier, and the notifications that cause the function to evaluate your AWS resources. See Source Below.
        #[builder(into)]
        pub source: pulumi_gestalt_rust::InputOrOutput<
            super::super::types::cfg::RuleSource,
        >,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RuleResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The ARN of the config rule
        pub arn: pulumi_gestalt_rust::Output<String>,
        /// Description of the rule
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The modes the Config rule can be evaluated in. See Evaluation Mode for more details.
        pub evaluation_modes: pulumi_gestalt_rust::Output<
            Vec<super::super::types::cfg::RuleEvaluationMode>,
        >,
        /// A string in JSON format that is passed to the AWS Config rule Lambda function.
        pub input_parameters: pulumi_gestalt_rust::Output<Option<String>>,
        /// The maximum frequency with which AWS Config runs evaluations for a rule.
        pub maximum_execution_frequency: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the rule
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the config rule
        pub rule_id: pulumi_gestalt_rust::Output<String>,
        /// Scope defines which resources can trigger an evaluation for the rule. See Scope Below.
        pub scope: pulumi_gestalt_rust::Output<
            Option<super::super::types::cfg::RuleScope>,
        >,
        /// Source specifies the rule owner, the rule identifier, and the notifications that cause the function to evaluate your AWS resources. See Source Below.
        pub source: pulumi_gestalt_rust::Output<super::super::types::cfg::RuleSource>,
        /// A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
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
        args: RuleArgs,
    ) -> RuleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let evaluation_modes_binding = args.evaluation_modes.get_output(context);
        let input_parameters_binding = args.input_parameters.get_output(context);
        let maximum_execution_frequency_binding = args
            .maximum_execution_frequency
            .get_output(context);
        let name_binding = args.name.get_output(context);
        let scope_binding = args.scope.get_output(context);
        let source_binding = args.source.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:cfg/rule:Rule".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "evaluationModes".into(),
                    value: &evaluation_modes_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "inputParameters".into(),
                    value: &input_parameters_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "maximumExecutionFrequency".into(),
                    value: &maximum_execution_frequency_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "scope".into(),
                    value: &scope_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "source".into(),
                    value: &source_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: &tags_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        RuleResult {
            id: o.get_field("id"),
            arn: o.get_field("arn"),
            description: o.get_field("description"),
            evaluation_modes: o.get_field("evaluationModes"),
            input_parameters: o.get_field("inputParameters"),
            maximum_execution_frequency: o.get_field("maximumExecutionFrequency"),
            name: o.get_field("name"),
            rule_id: o.get_field("ruleId"),
            scope: o.get_field("scope"),
            source: o.get_field("source"),
            tags: o.get_field("tags"),
            tags_all: o.get_field("tagsAll"),
        }
    }
}
