/// Three different resources help you manage your IAM policy for Backup for GKE BackupPlan. Each of these resources serves a different use case:
///
/// * `gcp.gkebackup.BackupPlanIamPolicy`: Authoritative. Sets the IAM policy for the backupplan and replaces any existing policy already attached.
/// * `gcp.gkebackup.BackupPlanIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the backupplan are preserved.
/// * `gcp.gkebackup.BackupPlanIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the backupplan are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.gkebackup.BackupPlanIamPolicy`: Retrieves the IAM policy for the backupplan
///
/// > **Note:** `gcp.gkebackup.BackupPlanIamPolicy` **cannot** be used in conjunction with `gcp.gkebackup.BackupPlanIamBinding` and `gcp.gkebackup.BackupPlanIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.gkebackup.BackupPlanIamBinding` resources **can be** used in conjunction with `gcp.gkebackup.BackupPlanIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.gkebackup.BackupPlanIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:gkebackup:BackupPlanIamPolicy
///     properties:
///       project: ${basic.project}
///       location: ${basic.location}
///       name: ${basic.name}
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
/// ## gcp.gkebackup.BackupPlanIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = backup_plan_iam_binding::create(
///         "binding",
///         BackupPlanIamBindingArgs::builder()
///             .location("${basic.location}")
///             .members(vec!["user:jane@example.com",])
///             .name("${basic.name}")
///             .project("${basic.project}")
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.gkebackup.BackupPlanIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = backup_plan_iam_member::create(
///         "member",
///         BackupPlanIamMemberArgs::builder()
///             .location("${basic.location}")
///             .member("user:jane@example.com")
///             .name("${basic.name}")
///             .project("${basic.project}")
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
/// # IAM policy for Backup for GKE BackupPlan
/// Three different resources help you manage your IAM policy for Backup for GKE BackupPlan. Each of these resources serves a different use case:
///
/// * `gcp.gkebackup.BackupPlanIamPolicy`: Authoritative. Sets the IAM policy for the backupplan and replaces any existing policy already attached.
/// * `gcp.gkebackup.BackupPlanIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the backupplan are preserved.
/// * `gcp.gkebackup.BackupPlanIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the backupplan are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.gkebackup.BackupPlanIamPolicy`: Retrieves the IAM policy for the backupplan
///
/// > **Note:** `gcp.gkebackup.BackupPlanIamPolicy` **cannot** be used in conjunction with `gcp.gkebackup.BackupPlanIamBinding` and `gcp.gkebackup.BackupPlanIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.gkebackup.BackupPlanIamBinding` resources **can be** used in conjunction with `gcp.gkebackup.BackupPlanIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.gkebackup.BackupPlanIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:gkebackup:BackupPlanIamPolicy
///     properties:
///       project: ${basic.project}
///       location: ${basic.location}
///       name: ${basic.name}
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
/// ## gcp.gkebackup.BackupPlanIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = backup_plan_iam_binding::create(
///         "binding",
///         BackupPlanIamBindingArgs::builder()
///             .location("${basic.location}")
///             .members(vec!["user:jane@example.com",])
///             .name("${basic.name}")
///             .project("${basic.project}")
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.gkebackup.BackupPlanIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = backup_plan_iam_member::create(
///         "member",
///         BackupPlanIamMemberArgs::builder()
///             .location("${basic.location}")
///             .member("user:jane@example.com")
///             .name("${basic.name}")
///             .project("${basic.project}")
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
/// * projects/{{project}}/locations/{{location}}/backupPlans/{{name}}
///
/// * {{project}}/{{location}}/{{name}}
///
/// * {{location}}/{{name}}
///
/// * {{name}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Backup for GKE backupplan IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:gkebackup/backupPlanIamPolicy:BackupPlanIamPolicy editor "projects/{{project}}/locations/{{location}}/backupPlans/{{backup_plan}} roles/viewer user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:gkebackup/backupPlanIamPolicy:BackupPlanIamPolicy editor "projects/{{project}}/locations/{{location}}/backupPlans/{{backup_plan}} roles/viewer"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:gkebackup/backupPlanIamPolicy:BackupPlanIamPolicy editor projects/{{project}}/locations/{{location}}/backupPlans/{{backup_plan}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod backup_plan_iam_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct BackupPlanIamPolicyArgs {
        /// The region of the Backup Plan.
        /// Used to find the parent resource to bind the IAM policy to. If not specified,
        /// the value will be parsed from the identifier of the parent resource. If no location is provided in the parent identifier and no
        /// location is specified, it is taken from the provider configuration.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Used to find the parent resource to bind the IAM policy to
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        #[builder(into)]
        pub policy_data: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct BackupPlanIamPolicyResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The region of the Backup Plan.
        /// Used to find the parent resource to bind the IAM policy to. If not specified,
        /// the value will be parsed from the identifier of the parent resource. If no location is provided in the parent identifier and no
        /// location is specified, it is taken from the provider configuration.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// Used to find the parent resource to bind the IAM policy to
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        pub policy_data: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: BackupPlanIamPolicyArgs,
    ) -> BackupPlanIamPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let policy_data_binding = args.policy_data.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:gkebackup/backupPlanIamPolicy:BackupPlanIamPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyData".into(),
                    value: &policy_data_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        BackupPlanIamPolicyResult {
            id: o.get_field("id"),
            etag: o.get_field("etag"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            policy_data: o.get_field("policyData"),
            project: o.get_field("project"),
        }
    }
}
