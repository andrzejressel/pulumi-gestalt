/// Three different resources help you manage IAM policies on bigtable instances. Each of these resources serves a different use case:
///
/// * `gcp.bigtable.InstanceIamPolicy`: Authoritative. Sets the IAM policy for the instance and replaces any existing policy already attached.
/// * `gcp.bigtable.InstanceIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the instance are preserved.
/// * `gcp.bigtable.InstanceIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the instance are preserved.
///
/// > **Note:** `gcp.bigtable.InstanceIamPolicy` **cannot** be used in conjunction with `gcp.bigtable.InstanceIamBinding` and `gcp.bigtable.InstanceIamMember` or they will fight over what your policy should be. In addition, be careful not to accidentally unset ownership of the instance as `gcp.bigtable.InstanceIamPolicy` replaces the entire policy.
///
/// > **Note:** `gcp.bigtable.InstanceIamBinding` resources **can be** used in conjunction with `gcp.bigtable.InstanceIamMember` resources **only if** they do not grant privilege to the same role.
///
/// ## gcp.bigtable.InstanceIamPolicy
///
/// ```yaml
/// resources:
///   editor:
///     type: gcp:bigtable:InstanceIamPolicy
///     properties:
///       project: your-project
///       instance: your-bigtable-instance
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/bigtable.user
///             members:
///               - user:jane@example.com
/// ```
///
/// ## gcp.bigtable.InstanceIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let editor = instance_iam_binding::create(
///         "editor",
///         InstanceIamBindingArgs::builder()
///             .instance("your-bigtable-instance")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/bigtable.user")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.bigtable.InstanceIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let editor = instance_iam_member::create(
///         "editor",
///         InstanceIamMemberArgs::builder()
///             .instance("your-bigtable-instance")
///             .member("user:jane@example.com")
///             .role("roles/bigtable.user")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.bigtable.InstanceIamPolicy
///
/// ```yaml
/// resources:
///   editor:
///     type: gcp:bigtable:InstanceIamPolicy
///     properties:
///       project: your-project
///       instance: your-bigtable-instance
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/bigtable.user
///             members:
///               - user:jane@example.com
/// ```
///
/// ## gcp.bigtable.InstanceIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let editor = instance_iam_binding::create(
///         "editor",
///         InstanceIamBindingArgs::builder()
///             .instance("your-bigtable-instance")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/bigtable.user")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.bigtable.InstanceIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let editor = instance_iam_member::create(
///         "editor",
///         InstanceIamMemberArgs::builder()
///             .instance("your-bigtable-instance")
///             .member("user:jane@example.com")
///             .role("roles/bigtable.user")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ### Importing IAM policies
///
/// IAM policy imports use the `instance` identifier of the Bigtable Instance resource only. For example:
///
/// * `"projects/{project}/instances/{instance}"`
///
/// An `import` block (Terraform v1.5.0 and later) can be used to import IAM policies:
///
/// tf
///
/// import {
///
///   id = "projects/{project}/instances/{instance}"
///
///   to = google_bigtable_instance_iam_policy.default
///
/// }
///
/// The `pulumi import` command can also be used:
///
/// ```sh
/// $ pulumi import gcp:bigtable/instanceIamBinding:InstanceIamBinding default projects/{project}/instances/{instance}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod instance_iam_binding {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct InstanceIamBindingArgs {
        /// An [IAM Condition](https://cloud.google.com/iam/docs/conditions-overview) for a given binding. Structure is documented below.
        #[builder(into, default)]
        pub condition: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::bigtable::InstanceIamBindingCondition>,
        >,
        /// The name or relative resource id of the instance to manage IAM policies for.
        ///
        /// For `gcp.bigtable.InstanceIamMember` or `gcp.bigtable.InstanceIamBinding`:
        #[builder(into)]
        pub instance: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Identities that will be granted the privilege in `role`.
        /// Each entry can have one of the following values:
        /// * **allUsers**: A special identifier that represents anyone who is on the internet; with or without a Google account.
        /// * **allAuthenticatedUsers**: A special identifier that represents anyone who is authenticated with a Google account or a service account.
        /// * **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
        /// * **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
        /// * **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
        /// * **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
        #[builder(into)]
        pub members: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The role that should be applied. Only one
        /// `gcp.bigtable.InstanceIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`. Read more about roles [here](https://cloud.google.com/bigtable/docs/access-control#roles).
        #[builder(into)]
        pub role: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct InstanceIamBindingResult {
        /// An [IAM Condition](https://cloud.google.com/iam/docs/conditions-overview) for a given binding. Structure is documented below.
        pub condition: pulumi_gestalt_rust::Output<
            Option<super::super::types::bigtable::InstanceIamBindingCondition>,
        >,
        /// (Computed) The etag of the instances's IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The name or relative resource id of the instance to manage IAM policies for.
        ///
        /// For `gcp.bigtable.InstanceIamMember` or `gcp.bigtable.InstanceIamBinding`:
        pub instance: pulumi_gestalt_rust::Output<String>,
        /// Identities that will be granted the privilege in `role`.
        /// Each entry can have one of the following values:
        /// * **allUsers**: A special identifier that represents anyone who is on the internet; with or without a Google account.
        /// * **allAuthenticatedUsers**: A special identifier that represents anyone who is authenticated with a Google account or a service account.
        /// * **user:{emailid}**: An email address that represents a specific Google account. For example, alice@gmail.com or joe@example.com.
        /// * **serviceAccount:{emailid}**: An email address that represents a service account. For example, my-other-app@appspot.gserviceaccount.com.
        /// * **group:{emailid}**: An email address that represents a Google group. For example, admins@example.com.
        /// * **domain:{domain}**: A G Suite domain (primary, instead of alias) name that represents all the users of that domain. For example, google.com or example.com.
        pub members: pulumi_gestalt_rust::Output<Vec<String>>,
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The role that should be applied. Only one
        /// `gcp.bigtable.InstanceIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`. Read more about roles [here](https://cloud.google.com/bigtable/docs/access-control#roles).
        pub role: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: InstanceIamBindingArgs,
    ) -> InstanceIamBindingResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let condition_binding = args.condition.get_output(context);
        let instance_binding = args.instance.get_output(context);
        let members_binding = args.members.get_output(context);
        let project_binding = args.project.get_output(context);
        let role_binding = args.role.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:bigtable/instanceIamBinding:InstanceIamBinding".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "condition".into(),
                    value: &condition_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "instance".into(),
                    value: &instance_binding.drop_type(),
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
        InstanceIamBindingResult {
            condition: o.get_field("condition"),
            etag: o.get_field("etag"),
            instance: o.get_field("instance"),
            members: o.get_field("members"),
            project: o.get_field("project"),
            role: o.get_field("role"),
        }
    }
}
