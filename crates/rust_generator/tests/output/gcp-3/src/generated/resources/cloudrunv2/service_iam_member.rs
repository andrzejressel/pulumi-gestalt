/// Three different resources help you manage your IAM policy for Cloud Run (v2 API) Service. Each of these resources serves a different use case:
///
/// * `gcp.cloudrunv2.ServiceIamPolicy`: Authoritative. Sets the IAM policy for the service and replaces any existing policy already attached.
/// * `gcp.cloudrunv2.ServiceIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the service are preserved.
/// * `gcp.cloudrunv2.ServiceIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the service are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.cloudrunv2.ServiceIamPolicy`: Retrieves the IAM policy for the service
///
/// > **Note:** `gcp.cloudrunv2.ServiceIamPolicy` **cannot** be used in conjunction with `gcp.cloudrunv2.ServiceIamBinding` and `gcp.cloudrunv2.ServiceIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.cloudrunv2.ServiceIamBinding` resources **can be** used in conjunction with `gcp.cloudrunv2.ServiceIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.cloudrunv2.ServiceIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:cloudrunv2:ServiceIamPolicy
///     properties:
///       project: ${default.project}
///       location: ${default.location}
///       name: ${default.name}
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
/// ## gcp.cloudrunv2.ServiceIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = service_iam_binding::create(
///         "binding",
///         ServiceIamBindingArgs::builder()
///             .location("${default.location}")
///             .members(vec!["user:jane@example.com",])
///             .name("${default.name}")
///             .project("${default.project}")
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.cloudrunv2.ServiceIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = service_iam_member::create(
///         "member",
///         ServiceIamMemberArgs::builder()
///             .location("${default.location}")
///             .member("user:jane@example.com")
///             .name("${default.name}")
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
/// # IAM policy for Cloud Run (v2 API) Service
/// Three different resources help you manage your IAM policy for Cloud Run (v2 API) Service. Each of these resources serves a different use case:
///
/// * `gcp.cloudrunv2.ServiceIamPolicy`: Authoritative. Sets the IAM policy for the service and replaces any existing policy already attached.
/// * `gcp.cloudrunv2.ServiceIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the service are preserved.
/// * `gcp.cloudrunv2.ServiceIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the service are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.cloudrunv2.ServiceIamPolicy`: Retrieves the IAM policy for the service
///
/// > **Note:** `gcp.cloudrunv2.ServiceIamPolicy` **cannot** be used in conjunction with `gcp.cloudrunv2.ServiceIamBinding` and `gcp.cloudrunv2.ServiceIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.cloudrunv2.ServiceIamBinding` resources **can be** used in conjunction with `gcp.cloudrunv2.ServiceIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## gcp.cloudrunv2.ServiceIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:cloudrunv2:ServiceIamPolicy
///     properties:
///       project: ${default.project}
///       location: ${default.location}
///       name: ${default.name}
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
/// ## gcp.cloudrunv2.ServiceIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = service_iam_binding::create(
///         "binding",
///         ServiceIamBindingArgs::builder()
///             .location("${default.location}")
///             .members(vec!["user:jane@example.com",])
///             .name("${default.name}")
///             .project("${default.project}")
///             .role("roles/viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.cloudrunv2.ServiceIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = service_iam_member::create(
///         "member",
///         ServiceIamMemberArgs::builder()
///             .location("${default.location}")
///             .member("user:jane@example.com")
///             .name("${default.name}")
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
/// * projects/{{project}}/locations/{{location}}/services/{{name}}
///
/// * {{project}}/{{location}}/{{name}}
///
/// * {{location}}/{{name}}
///
/// * {{name}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Cloud Run (v2 API) service IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:cloudrunv2/serviceIamMember:ServiceIamMember editor "projects/{{project}}/locations/{{location}}/services/{{service}} roles/viewer user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:cloudrunv2/serviceIamMember:ServiceIamMember editor "projects/{{project}}/locations/{{location}}/services/{{service}} roles/viewer"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:cloudrunv2/serviceIamMember:ServiceIamMember editor projects/{{project}}/locations/{{location}}/services/{{service}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod service_iam_member {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ServiceIamMemberArgs {
        #[builder(into, default)]
        pub condition: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::cloudrunv2::ServiceIamMemberCondition>,
        >,
        /// The location of the cloud run service Used to find the parent resource to bind the IAM policy to. If not specified,
        /// the value will be parsed from the identifier of the parent resource. If no location is provided in the parent identifier and no
        /// location is specified, it is taken from the provider configuration.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
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
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The role that should be applied. Only one
        /// `gcp.cloudrunv2.ServiceIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        #[builder(into)]
        pub role: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ServiceIamMemberResult {
        pub condition: pulumi_gestalt_rust::Output<
            Option<super::super::types::cloudrunv2::ServiceIamMemberCondition>,
        >,
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The location of the cloud run service Used to find the parent resource to bind the IAM policy to. If not specified,
        /// the value will be parsed from the identifier of the parent resource. If no location is provided in the parent identifier and no
        /// location is specified, it is taken from the provider configuration.
        pub location: pulumi_gestalt_rust::Output<String>,
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
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// The role that should be applied. Only one
        /// `gcp.cloudrunv2.ServiceIamBinding` can be used per role. Note that custom roles must be of the format
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
        args: ServiceIamMemberArgs,
    ) -> ServiceIamMemberResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let condition_binding = args.condition.get_output(context);
        let location_binding = args.location.get_output(context);
        let member_binding = args.member.get_output(context);
        let name_binding = args.name.get_output(context);
        let project_binding = args.project.get_output(context);
        let role_binding = args.role.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:cloudrunv2/serviceIamMember:ServiceIamMember".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "condition".into(),
                    value: &condition_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: &location_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "member".into(),
                    value: &member_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
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
        ServiceIamMemberResult {
            condition: o.get_field("condition"),
            etag: o.get_field("etag"),
            location: o.get_field("location"),
            member: o.get_field("member"),
            name: o.get_field("name"),
            project: o.get_field("project"),
            role: o.get_field("role"),
        }
    }
}
