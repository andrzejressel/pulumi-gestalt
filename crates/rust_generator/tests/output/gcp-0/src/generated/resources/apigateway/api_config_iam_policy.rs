/// Three different resources help you manage your IAM policy for API Gateway ApiConfig. Each of these resources serves a different use case:
///
/// * `gcp.apigateway.ApiConfigIamPolicy`: Authoritative. Sets the IAM policy for the apiconfig and replaces any existing policy already attached.
/// * `gcp.apigateway.ApiConfigIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the apiconfig are preserved.
/// * `gcp.apigateway.ApiConfigIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the apiconfig are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.apigateway.ApiConfigIamPolicy`: Retrieves the IAM policy for the apiconfig
///
/// > **Note:** `gcp.apigateway.ApiConfigIamPolicy` **cannot** be used in conjunction with `gcp.apigateway.ApiConfigIamBinding` and `gcp.apigateway.ApiConfigIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.apigateway.ApiConfigIamBinding` resources **can be** used in conjunction with `gcp.apigateway.ApiConfigIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## google\_api\_gateway\_api\_config\_iam\_policy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:apigateway:ApiConfigIamPolicy
///     properties:
///       api: ${apiCfg.api}
///       apiConfig: ${apiCfg.apiConfigId}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/apigateway.viewer
///             members:
///               - user:jane@example.com
/// ```
///
/// ## gcp.apigateway.ApiConfigIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = api_config_iam_binding::create(
///         "binding",
///         ApiConfigIamBindingArgs::builder()
///             .api("${apiCfg.api}")
///             .api_config("${apiCfg.apiConfigId}")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/apigateway.viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.apigateway.ApiConfigIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = api_config_iam_member::create(
///         "member",
///         ApiConfigIamMemberArgs::builder()
///             .api("${apiCfg.api}")
///             .api_config("${apiCfg.apiConfigId}")
///             .member("user:jane@example.com")
///             .role("roles/apigateway.viewer")
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
/// # IAM policy for API Gateway ApiConfig
/// Three different resources help you manage your IAM policy for API Gateway ApiConfig. Each of these resources serves a different use case:
///
/// * `gcp.apigateway.ApiConfigIamPolicy`: Authoritative. Sets the IAM policy for the apiconfig and replaces any existing policy already attached.
/// * `gcp.apigateway.ApiConfigIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the apiconfig are preserved.
/// * `gcp.apigateway.ApiConfigIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the apiconfig are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.apigateway.ApiConfigIamPolicy`: Retrieves the IAM policy for the apiconfig
///
/// > **Note:** `gcp.apigateway.ApiConfigIamPolicy` **cannot** be used in conjunction with `gcp.apigateway.ApiConfigIamBinding` and `gcp.apigateway.ApiConfigIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.apigateway.ApiConfigIamBinding` resources **can be** used in conjunction with `gcp.apigateway.ApiConfigIamMember` resources **only if** they do not grant privilege to the same role.
///
///
///
/// ## google\_api\_gateway\_api\_config\_iam\_policy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:apigateway:ApiConfigIamPolicy
///     properties:
///       api: ${apiCfg.api}
///       apiConfig: ${apiCfg.apiConfigId}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/apigateway.viewer
///             members:
///               - user:jane@example.com
/// ```
///
/// ## gcp.apigateway.ApiConfigIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = api_config_iam_binding::create(
///         "binding",
///         ApiConfigIamBindingArgs::builder()
///             .api("${apiCfg.api}")
///             .api_config("${apiCfg.apiConfigId}")
///             .members(vec!["user:jane@example.com",])
///             .role("roles/apigateway.viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.apigateway.ApiConfigIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = api_config_iam_member::create(
///         "member",
///         ApiConfigIamMemberArgs::builder()
///             .api("${apiCfg.api}")
///             .api_config("${apiCfg.apiConfigId}")
///             .member("user:jane@example.com")
///             .role("roles/apigateway.viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// For all import syntaxes, the "resource in question" can take any of the following forms:
///
/// * projects/{{project}}/locations/global/apis/{{api}}/configs/{{api_config}}
///
/// * {{project}}/{{api}}/{{api_config}}
///
/// * {{api}}/{{api_config}}
///
/// * {{api_config}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// API Gateway apiconfig IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:apigateway/apiConfigIamPolicy:ApiConfigIamPolicy editor "projects/{{project}}/locations/global/apis/{{api}}/configs/{{api_config}} roles/apigateway.viewer user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:apigateway/apiConfigIamPolicy:ApiConfigIamPolicy editor "projects/{{project}}/locations/global/apis/{{api}}/configs/{{api_config}} roles/apigateway.viewer"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:apigateway/apiConfigIamPolicy:ApiConfigIamPolicy editor projects/{{project}}/locations/global/apis/{{api}}/configs/{{api_config}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod api_config_iam_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ApiConfigIamPolicyArgs {
        /// The API to attach the config to.
        /// Used to find the parent resource to bind the IAM policy to
        #[builder(into)]
        pub api: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into)]
        pub api_config: pulumi_gestalt_rust::InputOrOutput<String>,
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
    pub struct ApiConfigIamPolicyResult {
        /// The API to attach the config to.
        /// Used to find the parent resource to bind the IAM policy to
        pub api: pulumi_gestalt_rust::Output<String>,
        pub api_config: pulumi_gestalt_rust::Output<String>,
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
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
        args: ApiConfigIamPolicyArgs,
    ) -> ApiConfigIamPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let api_binding = args.api.get_output(context);
        let api_config_binding = args.api_config.get_output(context);
        let policy_data_binding = args.policy_data.get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:apigateway/apiConfigIamPolicy:ApiConfigIamPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "api".into(),
                    value: &api_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "apiConfig".into(),
                    value: &api_config_binding.drop_type(),
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
        ApiConfigIamPolicyResult {
            api: o.get_field("api"),
            api_config: o.get_field("apiConfig"),
            etag: o.get_field("etag"),
            policy_data: o.get_field("policyData"),
            project: o.get_field("project"),
        }
    }
}
