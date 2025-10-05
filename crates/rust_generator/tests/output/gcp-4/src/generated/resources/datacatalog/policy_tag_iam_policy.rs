/// Three different resources help you manage your IAM policy for Data catalog PolicyTag. Each of these resources serves a different use case:
///
/// * `gcp.datacatalog.PolicyTagIamPolicy`: Authoritative. Sets the IAM policy for the policytag and replaces any existing policy already attached.
/// * `gcp.datacatalog.PolicyTagIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the policytag are preserved.
/// * `gcp.datacatalog.PolicyTagIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the policytag are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.datacatalog.PolicyTagIamPolicy`: Retrieves the IAM policy for the policytag
///
/// > **Note:** `gcp.datacatalog.PolicyTagIamPolicy` **cannot** be used in conjunction with `gcp.datacatalog.PolicyTagIamBinding` and `gcp.datacatalog.PolicyTagIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.datacatalog.PolicyTagIamBinding` resources **can be** used in conjunction with `gcp.datacatalog.PolicyTagIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.datacatalog.PolicyTagIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:datacatalog:PolicyTagIamPolicy
///     properties:
///       policyTag: ${basicPolicyTag.name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/viewer
///             members:
///               - user:jane@example.com
/// ```
///
/// ## gcp.datacatalog.PolicyTagIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = policy_tag_iam_binding::create(
///         "binding",
///         PolicyTagIamBindingArgs::builder()
///             .members(vec!["user:jane@example.com",])
///             .policy_tag("${basicPolicyTag.name}")
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.datacatalog.PolicyTagIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = policy_tag_iam_member::create(
///         "member",
///         PolicyTagIamMemberArgs::builder()
///             .member("user:jane@example.com")
///             .policy_tag("${basicPolicyTag.name}")
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
///
/// ## > **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
/// -
///
/// # IAM policy for Data catalog PolicyTag
/// Three different resources help you manage your IAM policy for Data catalog PolicyTag. Each of these resources serves a different use case:
///
/// * `gcp.datacatalog.PolicyTagIamPolicy`: Authoritative. Sets the IAM policy for the policytag and replaces any existing policy already attached.
/// * `gcp.datacatalog.PolicyTagIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the policytag are preserved.
/// * `gcp.datacatalog.PolicyTagIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the policytag are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.datacatalog.PolicyTagIamPolicy`: Retrieves the IAM policy for the policytag
///
/// > **Note:** `gcp.datacatalog.PolicyTagIamPolicy` **cannot** be used in conjunction with `gcp.datacatalog.PolicyTagIamBinding` and `gcp.datacatalog.PolicyTagIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.datacatalog.PolicyTagIamBinding` resources **can be** used in conjunction with `gcp.datacatalog.PolicyTagIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.datacatalog.PolicyTagIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:datacatalog:PolicyTagIamPolicy
///     properties:
///       policyTag: ${basicPolicyTag.name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/viewer
///             members:
///               - user:jane@example.com
/// ```
///
/// ## gcp.datacatalog.PolicyTagIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = policy_tag_iam_binding::create(
///         "binding",
///         PolicyTagIamBindingArgs::builder()
///             .members(vec!["user:jane@example.com",])
///             .policy_tag("${basicPolicyTag.name}")
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.datacatalog.PolicyTagIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = policy_tag_iam_member::create(
///         "member",
///         PolicyTagIamMemberArgs::builder()
///             .member("user:jane@example.com")
///             .policy_tag("${basicPolicyTag.name}")
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// For all import syntaxes, the "resource in question" can take any of the following forms:
///
/// * {{policy_tag}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Data catalog policytag IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:datacatalog/policyTagIamPolicy:PolicyTagIamPolicy editor "{{policy_tag}} roles/viewer user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:datacatalog/policyTagIamPolicy:PolicyTagIamPolicy editor "{{policy_tag}} roles/viewer"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:datacatalog/policyTagIamPolicy:PolicyTagIamPolicy editor {{policy_tag}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod policy_tag_iam_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicyTagIamPolicyArgs {
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        #[builder(into)]
        pub policy_data: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Used to find the parent resource to bind the IAM policy to
        #[builder(into)]
        pub policy_tag: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PolicyTagIamPolicyResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        pub policy_data: pulumi_gestalt_rust::Output<String>,
        /// Used to find the parent resource to bind the IAM policy to
        pub policy_tag: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PolicyTagIamPolicyArgs,
    ) -> PolicyTagIamPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let policy_data_binding = args.policy_data.get_output(context);
        let policy_tag_binding = args.policy_tag.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:datacatalog/policyTagIamPolicy:PolicyTagIamPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyData".into(),
                    value: &policy_data_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyTag".into(),
                    value: &policy_tag_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        PolicyTagIamPolicyResult {
            id: o.get_field("id"),
            etag: o.get_field("etag"),
            policy_data: o.get_field("policyData"),
            policy_tag: o.get_field("policyTag"),
        }
    }
}
