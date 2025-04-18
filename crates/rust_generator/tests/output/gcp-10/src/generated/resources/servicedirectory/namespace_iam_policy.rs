/// Three different resources help you manage your IAM policy for Service Directory Namespace. Each of these resources serves a different use case:
///
/// * `gcp.servicedirectory.NamespaceIamPolicy`: Authoritative. Sets the IAM policy for the namespace and replaces any existing policy already attached.
/// * `gcp.servicedirectory.NamespaceIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the namespace are preserved.
/// * `gcp.servicedirectory.NamespaceIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the namespace are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.servicedirectory.NamespaceIamPolicy`: Retrieves the IAM policy for the namespace
///
/// > **Note:** `gcp.servicedirectory.NamespaceIamPolicy` **cannot** be used in conjunction with `gcp.servicedirectory.NamespaceIamBinding` and `gcp.servicedirectory.NamespaceIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.servicedirectory.NamespaceIamBinding` resources **can be** used in conjunction with `gcp.servicedirectory.NamespaceIamMember` resources **only if** they do not grant privilege to the same role.
///
/// ## google\_service\_directory\_namespace\_iam\_policy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:servicedirectory:NamespaceIamPolicy
///     properties:
///       name: ${example.name}
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
/// ## gcp.servicedirectory.NamespaceIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = namespace_iam_binding::create(
///         "binding",
///         NamespaceIamBindingArgs::builder()
///             .members(vec!["user:jane@example.com",])
///             .name("${example.name}")
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.servicedirectory.NamespaceIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = namespace_iam_member::create(
///         "member",
///         NamespaceIamMemberArgs::builder()
///             .member("user:jane@example.com")
///             .name("${example.name}")
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
/// # IAM policy for Service Directory Namespace
/// Three different resources help you manage your IAM policy for Service Directory Namespace. Each of these resources serves a different use case:
///
/// * `gcp.servicedirectory.NamespaceIamPolicy`: Authoritative. Sets the IAM policy for the namespace and replaces any existing policy already attached.
/// * `gcp.servicedirectory.NamespaceIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the namespace are preserved.
/// * `gcp.servicedirectory.NamespaceIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the namespace are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.servicedirectory.NamespaceIamPolicy`: Retrieves the IAM policy for the namespace
///
/// > **Note:** `gcp.servicedirectory.NamespaceIamPolicy` **cannot** be used in conjunction with `gcp.servicedirectory.NamespaceIamBinding` and `gcp.servicedirectory.NamespaceIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.servicedirectory.NamespaceIamBinding` resources **can be** used in conjunction with `gcp.servicedirectory.NamespaceIamMember` resources **only if** they do not grant privilege to the same role.
///
/// ## google\_service\_directory\_namespace\_iam\_policy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:servicedirectory:NamespaceIamPolicy
///     properties:
///       name: ${example.name}
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
/// ## gcp.servicedirectory.NamespaceIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = namespace_iam_binding::create(
///         "binding",
///         NamespaceIamBindingArgs::builder()
///             .members(vec!["user:jane@example.com",])
///             .name("${example.name}")
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.servicedirectory.NamespaceIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = namespace_iam_member::create(
///         "member",
///         NamespaceIamMemberArgs::builder()
///             .member("user:jane@example.com")
///             .name("${example.name}")
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
/// * projects/{{project}}/locations/{{location}}/namespaces/{{namespace_id}}
///
/// * {{project}}/{{location}}/{{namespace_id}}
///
/// * {{location}}/{{namespace_id}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Service Directory namespace IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:servicedirectory/namespaceIamPolicy:NamespaceIamPolicy editor "projects/{{project}}/locations/{{location}}/namespaces/{{namespace_id}} roles/viewer user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:servicedirectory/namespaceIamPolicy:NamespaceIamPolicy editor "projects/{{project}}/locations/{{location}}/namespaces/{{namespace_id}} roles/viewer"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:servicedirectory/namespaceIamPolicy:NamespaceIamPolicy editor projects/{{project}}/locations/{{location}}/namespaces/{{namespace_id}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod namespace_iam_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NamespaceIamPolicyArgs {
        /// Used to find the parent resource to bind the IAM policy to
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        #[builder(into)]
        pub policy_data: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct NamespaceIamPolicyResult {
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// Used to find the parent resource to bind the IAM policy to
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        pub policy_data: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: NamespaceIamPolicyArgs,
    ) -> NamespaceIamPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let policy_data_binding = args.policy_data.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:servicedirectory/namespaceIamPolicy:NamespaceIamPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyData".into(),
                    value: &policy_data_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        NamespaceIamPolicyResult {
            etag: o.get_field("etag"),
            name: o.get_field("name"),
            policy_data: o.get_field("policyData"),
        }
    }
}
