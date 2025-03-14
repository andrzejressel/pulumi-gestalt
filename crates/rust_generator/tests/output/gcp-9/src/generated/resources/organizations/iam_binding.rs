/// Allows creation and management of a single binding within IAM policy for
/// an existing Google Cloud Platform Organization.
///
/// > **Note:** This resource __must not__ be used in conjunction with
///    `gcp.organizations.IAMMember` for the __same role__ or they will fight over
///    what your policy should be.
///
/// > **Note:** On create, this resource will overwrite members of any existing roles.
///     Use `pulumi import` and inspect the `output to ensure
///     your existing members are preserved.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = iam_binding::create(
///         "binding",
///         IamBindingArgs::builder()
///             .members(vec!["user:alice@gmail.com",])
///             .org_id("123456789")
///             .role("roles/browser")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// IAM binding imports use space-delimited identifiers; first the resource in question and then the role.  These bindings can be imported using the `org_id` and role, e.g.
///
/// ```sh
/// $ pulumi import gcp:organizations/iAMBinding:IAMBinding my_org "your-org-id roles/viewer"
/// ```
///
/// -> **Custom Roles**: If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod iam_binding {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IAMBindingArgs {
        #[builder(into, default)]
        pub condition: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::organizations::IamBindingCondition>,
        >,
        /// A list of users that the role should apply to. For more details on format and restrictions see https://cloud.google.com/billing/reference/rest/v1/Policy#Binding
        #[builder(into)]
        pub members: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The numeric ID of the organization in which you want to create a custom role.
        #[builder(into)]
        pub org_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The role that should be applied. Only one
        /// `gcp.organizations.IAMBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        #[builder(into)]
        pub role: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct IAMBindingResult {
        pub condition: pulumi_gestalt_rust::Output<
            Option<super::super::types::organizations::IamBindingCondition>,
        >,
        /// (Computed) The etag of the organization's IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// A list of users that the role should apply to. For more details on format and restrictions see https://cloud.google.com/billing/reference/rest/v1/Policy#Binding
        pub members: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The numeric ID of the organization in which you want to create a custom role.
        pub org_id: pulumi_gestalt_rust::Output<String>,
        /// The role that should be applied. Only one
        /// `gcp.organizations.IAMBinding` can be used per role. Note that custom roles must be of the format
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
        args: IAMBindingArgs,
    ) -> IAMBindingResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let condition_binding = args.condition.get_output(context);
        let members_binding = args.members.get_output(context);
        let org_id_binding = args.org_id.get_output(context);
        let role_binding = args.role.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:organizations/iAMBinding:IAMBinding".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "condition".into(),
                    value: &condition_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "members".into(),
                    value: &members_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "orgId".into(),
                    value: &org_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "role".into(),
                    value: &role_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        IAMBindingResult {
            condition: o.get_field("condition"),
            etag: o.get_field("etag"),
            members: o.get_field("members"),
            org_id: o.get_field("orgId"),
            role: o.get_field("role"),
        }
    }
}
