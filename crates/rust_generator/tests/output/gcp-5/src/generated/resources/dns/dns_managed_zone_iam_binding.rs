/// Three different resources help you manage your IAM policy for Cloud DNS ManagedZone. Each of these resources serves a different use case:
///
/// * `gcp.dns.DnsManagedZoneIamPolicy`: Authoritative. Sets the IAM policy for the managedzone and replaces any existing policy already attached.
/// * `gcp.dns.DnsManagedZoneIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the managedzone are preserved.
/// * `gcp.dns.DnsManagedZoneIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the managedzone are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.dns.DnsManagedZoneIamPolicy`: Retrieves the IAM policy for the managedzone
///
/// > **Note:** `gcp.dns.DnsManagedZoneIamPolicy` **cannot** be used in conjunction with `gcp.dns.DnsManagedZoneIamBinding` and `gcp.dns.DnsManagedZoneIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.dns.DnsManagedZoneIamBinding` resources **can be** used in conjunction with `gcp.dns.DnsManagedZoneIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.dns.DnsManagedZoneIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:dns:DnsManagedZoneIamPolicy
///     properties:
///       project: ${default.project}
///       managedZone: ${default.name}
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
/// ## gcp.dns.DnsManagedZoneIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = dns_managed_zone_iam_binding::create(
///         "binding",
///         DnsManagedZoneIamBindingArgs::builder()
///             .managed_zone("${default.name}")
///             .members(vec!["user:jane@example.com",])
///             .project("${default.project}")
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.dns.DnsManagedZoneIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = dns_managed_zone_iam_member::create(
///         "member",
///         DnsManagedZoneIamMemberArgs::builder()
///             .managed_zone("${default.name}")
///             .member("user:jane@example.com")
///             .project("${default.project}")
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
///
/// ## This resource supports User Project Overrides.
///
/// -
///
/// # IAM policy for Cloud DNS ManagedZone
/// Three different resources help you manage your IAM policy for Cloud DNS ManagedZone. Each of these resources serves a different use case:
///
/// * `gcp.dns.DnsManagedZoneIamPolicy`: Authoritative. Sets the IAM policy for the managedzone and replaces any existing policy already attached.
/// * `gcp.dns.DnsManagedZoneIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the managedzone are preserved.
/// * `gcp.dns.DnsManagedZoneIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the managedzone are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.dns.DnsManagedZoneIamPolicy`: Retrieves the IAM policy for the managedzone
///
/// > **Note:** `gcp.dns.DnsManagedZoneIamPolicy` **cannot** be used in conjunction with `gcp.dns.DnsManagedZoneIamBinding` and `gcp.dns.DnsManagedZoneIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.dns.DnsManagedZoneIamBinding` resources **can be** used in conjunction with `gcp.dns.DnsManagedZoneIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.dns.DnsManagedZoneIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:dns:DnsManagedZoneIamPolicy
///     properties:
///       project: ${default.project}
///       managedZone: ${default.name}
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
/// ## gcp.dns.DnsManagedZoneIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = dns_managed_zone_iam_binding::create(
///         "binding",
///         DnsManagedZoneIamBindingArgs::builder()
///             .managed_zone("${default.name}")
///             .members(vec!["user:jane@example.com",])
///             .project("${default.project}")
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.dns.DnsManagedZoneIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = dns_managed_zone_iam_member::create(
///         "member",
///         DnsManagedZoneIamMemberArgs::builder()
///             .managed_zone("${default.name}")
///             .member("user:jane@example.com")
///             .project("${default.project}")
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
/// * projects/{{project}}/managedZones/{{managed_zone}}
///
/// * {{project}}/{{managed_zone}}
///
/// * {{managed_zone}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Cloud DNS managedzone IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:dns/dnsManagedZoneIamBinding:DnsManagedZoneIamBinding editor "projects/{{project}}/managedZones/{{managed_zone}} roles/viewer user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:dns/dnsManagedZoneIamBinding:DnsManagedZoneIamBinding editor "projects/{{project}}/managedZones/{{managed_zone}} roles/viewer"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:dns/dnsManagedZoneIamBinding:DnsManagedZoneIamBinding editor projects/{{project}}/managedZones/{{managed_zone}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod dns_managed_zone_iam_binding {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct DnsManagedZoneIamBindingArgs {
        #[builder(into, default)]
        pub condition: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::dns::DnsManagedZoneIamBindingCondition>,
        >,
        /// Used to find the parent resource to bind the IAM policy to
        #[builder(into)]
        pub managed_zone: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Identities that will be granted the privilege in `role`.
        /// Each entry can have one of the following values:
        /// * **allUsers**: A special identifier that represents anyone who is on the internet; with or without a Google account.
        /// * **allAuthenticatedUsers**: A special identifier that represents anyone who is authenticated with a Google account or a service account.
        /// * **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
        /// * **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
        /// * **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
        /// * **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
        /// * **projectOwner:projectid**: Owners of the given project. For example, "projectOwner:my-example-project"
        /// * **projectEditor:projectid**: Editors of the given project. For example, "projectEditor:my-example-project"
        /// * **projectViewer:projectid**: Viewers of the given project. For example, "projectViewer:my-example-project"
        #[builder(into)]
        pub members: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The role that should be applied. Only one
        /// `gcp.dns.DnsManagedZoneIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        #[builder(into)]
        pub role: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct DnsManagedZoneIamBindingResult {
        pub condition: pulumi_gestalt_rust::Output<
            Option<super::super::types::dns::DnsManagedZoneIamBindingCondition>,
        >,
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// Used to find the parent resource to bind the IAM policy to
        pub managed_zone: pulumi_gestalt_rust::Output<String>,
        /// Identities that will be granted the privilege in `role`.
        /// Each entry can have one of the following values:
        /// * **allUsers**: A special identifier that represents anyone who is on the internet; with or without a Google account.
        /// * **allAuthenticatedUsers**: A special identifier that represents anyone who is authenticated with a Google account or a service account.
        /// * **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
        /// * **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
        /// * **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
        /// * **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
        /// * **projectOwner:projectid**: Owners of the given project. For example, "projectOwner:my-example-project"
        /// * **projectEditor:projectid**: Editors of the given project. For example, "projectEditor:my-example-project"
        /// * **projectViewer:projectid**: Viewers of the given project. For example, "projectViewer:my-example-project"
        pub members: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The role that should be applied. Only one
        /// `gcp.dns.DnsManagedZoneIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        pub role: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: DnsManagedZoneIamBindingArgs,
    ) -> DnsManagedZoneIamBindingResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let condition_binding = args.condition.get_output(context);
        let managed_zone_binding = args.managed_zone.get_output(context);
        let members_binding = args.members.get_output(context);
        let project_binding = args.project.get_output(context);
        let role_binding = args.role.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:dns/dnsManagedZoneIamBinding:DnsManagedZoneIamBinding".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "condition".into(),
                    value: &condition_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "managedZone".into(),
                    value: &managed_zone_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "members".into(),
                    value: &members_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "role".into(),
                    value: &role_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        DnsManagedZoneIamBindingResult {
            condition: o.get_field("condition"),
            etag: o.get_field("etag"),
            managed_zone: o.get_field("managedZone"),
            members: o.get_field("members"),
            project: o.get_field("project"),
            role: o.get_field("role"),
        }
    }
}
