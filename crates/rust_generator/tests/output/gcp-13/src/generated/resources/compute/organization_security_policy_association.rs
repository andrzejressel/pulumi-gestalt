/// An association for the OrganizationSecurityPolicy.
///
/// To get more information about OrganizationSecurityPolicyAssociation, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/beta/organizationSecurityPolicies/addAssociation)
/// * How-to Guides
///     * [Associating a policy with the organization or folder](https://cloud.google.com/vpc/docs/using-firewall-policies#associate)
///
/// ## Example Usage
///
/// ### Organization Security Policy Association Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let policy = organization_security_policy::create(
///         "policy",
///         OrganizationSecurityPolicyArgs::builder()
///             .display_name("tf-test")
///             .parent("${securityPolicyTarget.name}")
///             .build_struct(),
///     );
///     let policyOrganizationSecurityPolicyAssociation = organization_security_policy_association::create(
///         "policyOrganizationSecurityPolicyAssociation",
///         OrganizationSecurityPolicyAssociationArgs::builder()
///             .attachment_id("${policy.parent}")
///             .name("tf-test")
///             .policy_id("${policy.id}")
///             .build_struct(),
///     );
///     let policyOrganizationSecurityPolicyRule = organization_security_policy_rule::create(
///         "policyOrganizationSecurityPolicyRule",
///         OrganizationSecurityPolicyRuleArgs::builder()
///             .action("allow")
///             .direction("INGRESS")
///             .enable_logging(true)
///             .match_(
///                 OrganizationSecurityPolicyRuleMatch::builder()
///                     .config(
///                         OrganizationSecurityPolicyRuleMatchConfig::builder()
///                             .layer4Configs(
///                                 vec![
///                                     OrganizationSecurityPolicyRuleMatchConfigLayer4Config::builder()
///                                     .ipProtocol("tcp").ports(vec!["22",]).build_struct(),
///                                     OrganizationSecurityPolicyRuleMatchConfigLayer4Config::builder()
///                                     .ipProtocol("icmp").build_struct(),
///                                 ],
///                             )
///                             .srcIpRanges(vec!["192.168.0.0/16", "10.0.0.0/8",])
///                             .build_struct(),
///                     )
///                     .build_struct(),
///             )
///             .policy_id("${policy.id}")
///             .priority(100)
///             .build_struct(),
///     );
///     let securityPolicyTarget = folder::create(
///         "securityPolicyTarget",
///         FolderArgs::builder()
///             .deletion_protection(false)
///             .display_name("tf-test-secpol")
///             .parent("organizations/123456789")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// OrganizationSecurityPolicyAssociation can be imported using any of these accepted formats:
///
/// * `{{policy_id}}/association/{{name}}`
///
/// When using the `pulumi import` command, OrganizationSecurityPolicyAssociation can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/organizationSecurityPolicyAssociation:OrganizationSecurityPolicyAssociation default {{policy_id}}/association/{{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod organization_security_policy_association {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OrganizationSecurityPolicyAssociationArgs {
        /// The resource that the security policy is attached to.
        #[builder(into)]
        pub attachment_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name for an association.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The security policy ID of the association.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub policy_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct OrganizationSecurityPolicyAssociationResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// The resource that the security policy is attached to.
        pub attachment_id: pulumi_gestalt_rust::Output<String>,
        /// The display name of the security policy of the association.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// The name for an association.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The security policy ID of the association.
        ///
        ///
        /// - - -
        pub policy_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: OrganizationSecurityPolicyAssociationArgs,
    ) -> OrganizationSecurityPolicyAssociationResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let attachment_id_binding = args.attachment_id.get_output(context);
        let name_binding = args.name.get_output(context);
        let policy_id_binding = args.policy_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/organizationSecurityPolicyAssociation:OrganizationSecurityPolicyAssociation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "attachmentId".into(),
                    value: &attachment_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyId".into(),
                    value: &policy_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        OrganizationSecurityPolicyAssociationResult {
            id: o.get_field("id"),
            attachment_id: o.get_field("attachmentId"),
            display_name: o.get_field("displayName"),
            name: o.get_field("name"),
            policy_id: o.get_field("policyId"),
        }
    }
}
