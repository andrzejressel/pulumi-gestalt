/// Three different resources help you manage your IAM policy for Dataplex Task. Each of these resources serves a different use case:
///
/// * `gcp.dataplex.TaskIamPolicy`: Authoritative. Sets the IAM policy for the task and replaces any existing policy already attached.
/// * `gcp.dataplex.TaskIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the task are preserved.
/// * `gcp.dataplex.TaskIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the task are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.dataplex.TaskIamPolicy`: Retrieves the IAM policy for the task
///
/// > **Note:** `gcp.dataplex.TaskIamPolicy` **cannot** be used in conjunction with `gcp.dataplex.TaskIamBinding` and `gcp.dataplex.TaskIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.dataplex.TaskIamBinding` resources **can be** used in conjunction with `gcp.dataplex.TaskIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.dataplex.TaskIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:dataplex:TaskIamPolicy
///     properties:
///       project: ${example.project}
///       location: ${example.location}
///       lake: ${example.lake}
///       taskId: ${example.taskId}
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
/// ## gcp.dataplex.TaskIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = task_iam_binding::create(
///         "binding",
///         TaskIamBindingArgs::builder()
///             .lake("${example.lake}")
///             .location("${example.location}")
///             .members(vec!["user:jane@example.com",])
///             .project("${example.project}")
///             .role("roles/viewer")
///             .task_id("${example.taskId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.dataplex.TaskIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = task_iam_member::create(
///         "member",
///         TaskIamMemberArgs::builder()
///             .lake("${example.lake}")
///             .location("${example.location}")
///             .member("user:jane@example.com")
///             .project("${example.project}")
///             .role("roles/viewer")
///             .task_id("${example.taskId}")
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
/// # IAM policy for Dataplex Task
/// Three different resources help you manage your IAM policy for Dataplex Task. Each of these resources serves a different use case:
///
/// * `gcp.dataplex.TaskIamPolicy`: Authoritative. Sets the IAM policy for the task and replaces any existing policy already attached.
/// * `gcp.dataplex.TaskIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the task are preserved.
/// * `gcp.dataplex.TaskIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the task are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.dataplex.TaskIamPolicy`: Retrieves the IAM policy for the task
///
/// > **Note:** `gcp.dataplex.TaskIamPolicy` **cannot** be used in conjunction with `gcp.dataplex.TaskIamBinding` and `gcp.dataplex.TaskIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.dataplex.TaskIamBinding` resources **can be** used in conjunction with `gcp.dataplex.TaskIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.dataplex.TaskIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:dataplex:TaskIamPolicy
///     properties:
///       project: ${example.project}
///       location: ${example.location}
///       lake: ${example.lake}
///       taskId: ${example.taskId}
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
/// ## gcp.dataplex.TaskIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = task_iam_binding::create(
///         "binding",
///         TaskIamBindingArgs::builder()
///             .lake("${example.lake}")
///             .location("${example.location}")
///             .members(vec!["user:jane@example.com",])
///             .project("${example.project}")
///             .role("roles/viewer")
///             .task_id("${example.taskId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.dataplex.TaskIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = task_iam_member::create(
///         "member",
///         TaskIamMemberArgs::builder()
///             .lake("${example.lake}")
///             .location("${example.location}")
///             .member("user:jane@example.com")
///             .project("${example.project}")
///             .role("roles/viewer")
///             .task_id("${example.taskId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// For all import syntaxes, the "resource in question" can take any of the following forms:
///
/// * projects/{{project}}/locations/{{location}}/lakes/{{lake}}/tasks/{{task_id}}
///
/// * {{project}}/{{location}}/{{lake}}/{{task_id}}
///
/// * {{location}}/{{lake}}/{{task_id}}
///
/// * {{task_id}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Dataplex task IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:dataplex/taskIamPolicy:TaskIamPolicy editor "projects/{{project}}/locations/{{location}}/lakes/{{lake}}/tasks/{{task_id}} roles/viewer user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:dataplex/taskIamPolicy:TaskIamPolicy editor "projects/{{project}}/locations/{{location}}/lakes/{{lake}}/tasks/{{task_id}} roles/viewer"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:dataplex/taskIamPolicy:TaskIamPolicy editor projects/{{project}}/locations/{{location}}/lakes/{{lake}}/tasks/{{task_id}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod task_iam_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TaskIamPolicyArgs {
        /// The lake in which the task will be created in.
        /// Used to find the parent resource to bind the IAM policy to
        #[builder(into)]
        pub lake: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The location in which the task will be created in.
        /// Used to find the parent resource to bind the IAM policy to. If not specified,
        /// the value will be parsed from the identifier of the parent resource. If no location is provided in the parent identifier and no
        /// location is specified, it is taken from the provider configuration.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        #[builder(into)]
        pub policy_data: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into)]
        pub task_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct TaskIamPolicyResult {
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The lake in which the task will be created in.
        /// Used to find the parent resource to bind the IAM policy to
        pub lake: pulumi_gestalt_rust::Output<String>,
        /// The location in which the task will be created in.
        /// Used to find the parent resource to bind the IAM policy to. If not specified,
        /// the value will be parsed from the identifier of the parent resource. If no location is provided in the parent identifier and no
        /// location is specified, it is taken from the provider configuration.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        pub policy_data: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        pub task_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TaskIamPolicyArgs,
    ) -> TaskIamPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let lake_binding = args.lake.get_output(context);
        let location_binding = args.location.get_output(context);
        let policy_data_binding = args.policy_data.get_output(context);
        let project_binding = args.project.get_output(context);
        let task_id_binding = args.task_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:dataplex/taskIamPolicy:TaskIamPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "lake".into(),
                    value: &lake_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyData".into(),
                    value: &policy_data_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "taskId".into(),
                    value: &task_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        TaskIamPolicyResult {
            etag: o.get_field("etag"),
            lake: o.get_field("lake"),
            location: o.get_field("location"),
            policy_data: o.get_field("policyData"),
            project: o.get_field("project"),
            task_id: o.get_field("taskId"),
        }
    }
}
