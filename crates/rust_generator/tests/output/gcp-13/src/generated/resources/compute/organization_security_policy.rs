/// Organization security policies are used to control incoming/outgoing traffic.
///
/// To get more information about OrganizationSecurityPolicy, see:
///
/// * [API documentation](https://cloud.google.com/compute/docs/reference/rest/beta/organizationSecurityPolicies)
/// * How-to Guides
///     * [Creating a firewall policy](https://cloud.google.com/vpc/docs/using-firewall-policies#create-policy)
///
/// ## Example Usage
///
/// ### Organization Security Policy Basic
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
///             .parent("organizations/123456789")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// OrganizationSecurityPolicy can be imported using any of these accepted formats:
///
/// * `locations/global/securityPolicies/{{policy_id}}`
///
/// * `{{policy_id}}`
///
/// When using the `pulumi import` command, OrganizationSecurityPolicy can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:compute/organizationSecurityPolicy:OrganizationSecurityPolicy default locations/global/securityPolicies/{{policy_id}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:compute/organizationSecurityPolicy:OrganizationSecurityPolicy default {{policy_id}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod organization_security_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct OrganizationSecurityPolicyArgs {
        /// A textual description for the organization security policy.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A textual name of the security policy.
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The parent of this OrganizationSecurityPolicy in the Cloud Resource Hierarchy.
        /// Format: organizations/{organization_id} or folders/{folder_id}
        ///
        ///
        /// - - -
        #[builder(into)]
        pub parent: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type indicates the intended use of the security policy.
        /// For organization security policies, the only supported type
        /// is "FIREWALL".
        /// Default value is `FIREWALL`.
        /// Possible values are: `FIREWALL`.
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct OrganizationSecurityPolicyResult {
        /// A textual description for the organization security policy.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// A textual name of the security policy.
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// Fingerprint of this resource. This field is used internally during
        /// updates of this resource.
        pub fingerprint: pulumi_gestalt_rust::Output<String>,
        /// The parent of this OrganizationSecurityPolicy in the Cloud Resource Hierarchy.
        /// Format: organizations/{organization_id} or folders/{folder_id}
        ///
        ///
        /// - - -
        pub parent: pulumi_gestalt_rust::Output<String>,
        /// The unique identifier for the resource. This identifier is defined by the server.
        pub policy_id: pulumi_gestalt_rust::Output<String>,
        /// The type indicates the intended use of the security policy.
        /// For organization security policies, the only supported type
        /// is "FIREWALL".
        /// Default value is `FIREWALL`.
        /// Possible values are: `FIREWALL`.
        pub type_: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: OrganizationSecurityPolicyArgs,
    ) -> OrganizationSecurityPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let parent_binding = args.parent.get_output(context);
        let type__binding = args.type_.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:compute/organizationSecurityPolicy:OrganizationSecurityPolicy"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "parent".into(),
                    value: &parent_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        OrganizationSecurityPolicyResult {
            description: o.get_field("description"),
            display_name: o.get_field("displayName"),
            fingerprint: o.get_field("fingerprint"),
            parent: o.get_field("parent"),
            policy_id: o.get_field("policyId"),
            type_: o.get_field("type"),
        }
    }
}
