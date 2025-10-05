/// Manages Security Hub configuration policy associations.
///
/// > **NOTE:** This resource requires `aws.securityhub.OrganizationConfiguration` to be configured with type `CENTRAL`. More information about Security Hub central configuration and configuration policies can be found in the [How Security Hub configuration policies work](https://docs.aws.amazon.com/securityhub/latest/userguide/configuration-policies-overview.html) documentation.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let accountExample = configuration_policy_association::create(
///         "accountExample",
///         ConfigurationPolicyAssociationArgs::builder()
///             .policy_id("${exampleConfigurationPolicy.id}")
///             .target_id("123456789012")
///             .build_struct(),
///     );
///     let example = finding_aggregator::create(
///         "example",
///         FindingAggregatorArgs::builder().linking_mode("ALL_REGIONS").build_struct(),
///     );
///     let exampleConfigurationPolicy = configuration_policy::create(
///         "exampleConfigurationPolicy",
///         ConfigurationPolicyArgs::builder()
///             .configuration_policy(
///                 ConfigurationPolicyConfigurationPolicy::builder()
///                     .enabledStandardArns(
///                         vec![
///                             "arn:aws:securityhub:us-east-1::standards/aws-foundational-security-best-practices/v/1.0.0",
///                             "arn:aws:securityhub:::ruleset/cis-aws-foundations-benchmark/v/1.2.0",
///                         ],
///                     )
///                     .securityControlsConfiguration(
///                         ConfigurationPolicyConfigurationPolicySecurityControlsConfiguration::builder()
///                             .disabledControlIdentifiers(vec![])
///                             .build_struct(),
///                     )
///                     .serviceEnabled(true)
///                     .build_struct(),
///             )
///             .description("This is an example configuration policy")
///             .name("Example")
///             .build_struct(),
///     );
///     let exampleOrganizationConfiguration = organization_configuration::create(
///         "exampleOrganizationConfiguration",
///         OrganizationConfigurationArgs::builder()
///             .auto_enable(false)
///             .auto_enable_standards("NONE")
///             .organization_configuration(
///                 OrganizationConfigurationOrganizationConfiguration::builder()
///                     .configurationType("CENTRAL")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let ouExample = configuration_policy_association::create(
///         "ouExample",
///         ConfigurationPolicyAssociationArgs::builder()
///             .policy_id("${exampleConfigurationPolicy.id}")
///             .target_id("ou-abcd-12345678")
///             .build_struct(),
///     );
///     let rootExample = configuration_policy_association::create(
///         "rootExample",
///         ConfigurationPolicyAssociationArgs::builder()
///             .policy_id("${exampleConfigurationPolicy.id}")
///             .target_id("r-abcd")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import an existing Security Hub enabled account using the target id. For example:
///
/// ```sh
/// $ pulumi import aws:securityhub/configurationPolicyAssociation:ConfigurationPolicyAssociation example_account_association 123456789012
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod configuration_policy_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ConfigurationPolicyAssociationArgs {
        /// The universally unique identifier (UUID) of the configuration policy.
        #[builder(into)]
        pub policy_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The identifier of the target account, organizational unit, or the root to associate with the specified configuration.
        #[builder(into)]
        pub target_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ConfigurationPolicyAssociationResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The universally unique identifier (UUID) of the configuration policy.
        pub policy_id: pulumi_gestalt_rust::Output<String>,
        /// The identifier of the target account, organizational unit, or the root to associate with the specified configuration.
        pub target_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ConfigurationPolicyAssociationArgs,
    ) -> ConfigurationPolicyAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let policy_id_binding = args.policy_id.get_output(context);
        let target_id_binding = args.target_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:securityhub/configurationPolicyAssociation:ConfigurationPolicyAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyId".into(),
                    value: &policy_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "targetId".into(),
                    value: &target_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ConfigurationPolicyAssociationResult {
            id: o.get_field("id"),
            policy_id: o.get_field("policyId"),
            target_id: o.get_field("targetId"),
        }
    }
}
