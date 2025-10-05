/// Three different resources help you manage your IAM policy for Container Registry Note. Each of these resources serves a different use case:
///
/// * `gcp.containeranalysis.NoteIamPolicy`: Authoritative. Sets the IAM policy for the note and replaces any existing policy already attached.
/// * `gcp.containeranalysis.NoteIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the note are preserved.
/// * `gcp.containeranalysis.NoteIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the note are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.containeranalysis.NoteIamPolicy`: Retrieves the IAM policy for the note
///
/// > **Note:** `gcp.containeranalysis.NoteIamPolicy` **cannot** be used in conjunction with `gcp.containeranalysis.NoteIamBinding` and `gcp.containeranalysis.NoteIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.containeranalysis.NoteIamBinding` resources **can be** used in conjunction with `gcp.containeranalysis.NoteIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.containeranalysis.NoteIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:containeranalysis:NoteIamPolicy
///     properties:
///       project: ${note.project}
///       note: ${note.name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/containeranalysis.notes.occurrences.viewer
///             members:
///               - user:jane@example.com
/// ```
///
/// ## gcp.containeranalysis.NoteIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = note_iam_binding::create(
///         "binding",
///         NoteIamBindingArgs::builder()
///             .members(vec!["user:jane@example.com",])
///             .note("${note.name}")
///             .project("${note.project}")
///             .role("roles/containeranalysis.notes.occurrences.viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.containeranalysis.NoteIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = note_iam_member::create(
///         "member",
///         NoteIamMemberArgs::builder()
///             .member("user:jane@example.com")
///             .note("${note.name}")
///             .project("${note.project}")
///             .role("roles/containeranalysis.notes.occurrences.viewer")
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
/// # IAM policy for Container Registry Note
/// Three different resources help you manage your IAM policy for Container Registry Note. Each of these resources serves a different use case:
///
/// * `gcp.containeranalysis.NoteIamPolicy`: Authoritative. Sets the IAM policy for the note and replaces any existing policy already attached.
/// * `gcp.containeranalysis.NoteIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the note are preserved.
/// * `gcp.containeranalysis.NoteIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the note are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.containeranalysis.NoteIamPolicy`: Retrieves the IAM policy for the note
///
/// > **Note:** `gcp.containeranalysis.NoteIamPolicy` **cannot** be used in conjunction with `gcp.containeranalysis.NoteIamBinding` and `gcp.containeranalysis.NoteIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.containeranalysis.NoteIamBinding` resources **can be** used in conjunction with `gcp.containeranalysis.NoteIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.containeranalysis.NoteIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:containeranalysis:NoteIamPolicy
///     properties:
///       project: ${note.project}
///       note: ${note.name}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/containeranalysis.notes.occurrences.viewer
///             members:
///               - user:jane@example.com
/// ```
///
/// ## gcp.containeranalysis.NoteIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = note_iam_binding::create(
///         "binding",
///         NoteIamBindingArgs::builder()
///             .members(vec!["user:jane@example.com",])
///             .note("${note.name}")
///             .project("${note.project}")
///             .role("roles/containeranalysis.notes.occurrences.viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.containeranalysis.NoteIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = note_iam_member::create(
///         "member",
///         NoteIamMemberArgs::builder()
///             .member("user:jane@example.com")
///             .note("${note.name}")
///             .project("${note.project}")
///             .role("roles/containeranalysis.notes.occurrences.viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// For all import syntaxes, the "resource in question" can take any of the following forms:
///
/// * projects/{{project}}/notes/{{name}}
///
/// * {{project}}/{{name}}
///
/// * {{name}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Container Registry note IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:containeranalysis/noteIamMember:NoteIamMember editor "projects/{{project}}/notes/{{note}} roles/containeranalysis.notes.occurrences.viewer user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:containeranalysis/noteIamMember:NoteIamMember editor "projects/{{project}}/notes/{{note}} roles/containeranalysis.notes.occurrences.viewer"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:containeranalysis/noteIamMember:NoteIamMember editor projects/{{project}}/notes/{{note}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod note_iam_member {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct NoteIamMemberArgs {
        #[builder(into, default)]
        pub condition: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::containeranalysis::NoteIamMemberCondition>,
        >,
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
        pub member: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Used to find the parent resource to bind the IAM policy to
        #[builder(into)]
        pub note: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The role that should be applied. Only one
        /// `gcp.containeranalysis.NoteIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        #[builder(into)]
        pub role: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct NoteIamMemberResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub condition: pulumi_gestalt_rust::Output<
            Option<super::super::types::containeranalysis::NoteIamMemberCondition>,
        >,
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
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
        pub member: pulumi_gestalt_rust::Output<String>,
        /// Used to find the parent resource to bind the IAM policy to
        pub note: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The role that should be applied. Only one
        /// `gcp.containeranalysis.NoteIamBinding` can be used per role. Note that custom roles must be of the format
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
        args: NoteIamMemberArgs,
    ) -> NoteIamMemberResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let condition_binding = args.condition.get_output(context);
        let member_binding = args.member.get_output(context);
        let note_binding = args.note.get_output(context);
        let project_binding = args.project.get_output(context);
        let role_binding = args.role.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:containeranalysis/noteIamMember:NoteIamMember".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "condition".into(),
                    value: &condition_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "member".into(),
                    value: &member_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "note".into(),
                    value: &note_binding.drop_type(),
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
        NoteIamMemberResult {
            id: o.get_field("id"),
            condition: o.get_field("condition"),
            etag: o.get_field("etag"),
            member: o.get_field("member"),
            note: o.get_field("note"),
            project: o.get_field("project"),
            role: o.get_field("role"),
        }
    }
}
