/// Three different resources help you manage your IAM policy for API Gateway Gateway. Each of these resources serves a different use case:
///
/// * `gcp.apigateway.GatewayIamPolicy`: Authoritative. Sets the IAM policy for the gateway and replaces any existing policy already attached.
/// * `gcp.apigateway.GatewayIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the gateway are preserved.
/// * `gcp.apigateway.GatewayIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the gateway are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.apigateway.GatewayIamPolicy`: Retrieves the IAM policy for the gateway
///
/// > **Note:** `gcp.apigateway.GatewayIamPolicy` **cannot** be used in conjunction with `gcp.apigateway.GatewayIamBinding` and `gcp.apigateway.GatewayIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.apigateway.GatewayIamBinding` resources **can be** used in conjunction with `gcp.apigateway.GatewayIamMember` resources **only if** they do not grant privilege to the same role.
///
///
/// ## google\_api\_gateway\_gateway\_iam\_policy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:apigateway:GatewayIamPolicy
///     properties:
///       project: ${apiGw.project}
///       region: ${apiGw.region}
///       gateway: ${apiGw.gatewayId}
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
/// ## gcp.apigateway.GatewayIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = gateway_iam_binding::create(
///         "binding",
///         GatewayIamBindingArgs::builder()
///             .gateway("${apiGw.gatewayId}")
///             .members(vec!["user:jane@example.com",])
///             .project("${apiGw.project}")
///             .region("${apiGw.region}")
///             .role("roles/apigateway.viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.apigateway.GatewayIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = gateway_iam_member::create(
///         "member",
///         GatewayIamMemberArgs::builder()
///             .gateway("${apiGw.gatewayId}")
///             .member("user:jane@example.com")
///             .project("${apiGw.project}")
///             .region("${apiGw.region}")
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
/// # IAM policy for API Gateway Gateway
/// Three different resources help you manage your IAM policy for API Gateway Gateway. Each of these resources serves a different use case:
///
/// * `gcp.apigateway.GatewayIamPolicy`: Authoritative. Sets the IAM policy for the gateway and replaces any existing policy already attached.
/// * `gcp.apigateway.GatewayIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the gateway are preserved.
/// * `gcp.apigateway.GatewayIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the gateway are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.apigateway.GatewayIamPolicy`: Retrieves the IAM policy for the gateway
///
/// > **Note:** `gcp.apigateway.GatewayIamPolicy` **cannot** be used in conjunction with `gcp.apigateway.GatewayIamBinding` and `gcp.apigateway.GatewayIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.apigateway.GatewayIamBinding` resources **can be** used in conjunction with `gcp.apigateway.GatewayIamMember` resources **only if** they do not grant privilege to the same role.
///
///
/// ## google\_api\_gateway\_gateway\_iam\_policy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:apigateway:GatewayIamPolicy
///     properties:
///       project: ${apiGw.project}
///       region: ${apiGw.region}
///       gateway: ${apiGw.gatewayId}
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
/// ## gcp.apigateway.GatewayIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = gateway_iam_binding::create(
///         "binding",
///         GatewayIamBindingArgs::builder()
///             .gateway("${apiGw.gatewayId}")
///             .members(vec!["user:jane@example.com",])
///             .project("${apiGw.project}")
///             .region("${apiGw.region}")
///             .role("roles/apigateway.viewer")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## gcp.apigateway.GatewayIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = gateway_iam_member::create(
///         "member",
///         GatewayIamMemberArgs::builder()
///             .gateway("${apiGw.gatewayId}")
///             .member("user:jane@example.com")
///             .project("${apiGw.project}")
///             .region("${apiGw.region}")
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
/// * projects/{{project}}/locations/{{region}}/gateways/{{gateway}}
///
/// * {{project}}/{{region}}/{{gateway}}
///
/// * {{region}}/{{gateway}}
///
/// * {{gateway}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// API Gateway gateway IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:apigateway/gatewayIamBinding:GatewayIamBinding editor "projects/{{project}}/locations/{{region}}/gateways/{{gateway}} roles/apigateway.viewer user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:apigateway/gatewayIamBinding:GatewayIamBinding editor "projects/{{project}}/locations/{{region}}/gateways/{{gateway}} roles/apigateway.viewer"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:apigateway/gatewayIamBinding:GatewayIamBinding editor projects/{{project}}/locations/{{region}}/gateways/{{gateway}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod gateway_iam_binding {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GatewayIamBindingArgs {
        #[builder(into, default)]
        pub condition: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::apigateway::GatewayIamBindingCondition>,
        >,
        #[builder(into)]
        pub gateway: pulumi_gestalt_rust::InputOrOutput<String>,
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
        /// The region of the gateway for the API.
        /// Used to find the parent resource to bind the IAM policy to. If not specified,
        /// the value will be parsed from the identifier of the parent resource. If no region is provided in the parent identifier and no
        /// region is specified, it is taken from the provider configuration.
        #[builder(into, default)]
        pub region: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The role that should be applied. Only one
        /// `gcp.apigateway.GatewayIamBinding` can be used per role. Note that custom roles must be of the format
        /// `[projects|organizations]/{parent-name}/roles/{role-name}`.
        #[builder(into)]
        pub role: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct GatewayIamBindingResult {
        pub condition: pulumi_gestalt_rust::Output<
            Option<super::super::types::apigateway::GatewayIamBindingCondition>,
        >,
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        pub gateway: pulumi_gestalt_rust::Output<String>,
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
        /// The region of the gateway for the API.
        /// Used to find the parent resource to bind the IAM policy to. If not specified,
        /// the value will be parsed from the identifier of the parent resource. If no region is provided in the parent identifier and no
        /// region is specified, it is taken from the provider configuration.
        pub region: pulumi_gestalt_rust::Output<String>,
        /// The role that should be applied. Only one
        /// `gcp.apigateway.GatewayIamBinding` can be used per role. Note that custom roles must be of the format
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
        args: GatewayIamBindingArgs,
    ) -> GatewayIamBindingResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let condition_binding = args.condition.get_output(context);
        let gateway_binding = args.gateway.get_output(context);
        let members_binding = args.members.get_output(context);
        let project_binding = args.project.get_output(context);
        let region_binding = args.region.get_output(context);
        let role_binding = args.role.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:apigateway/gatewayIamBinding:GatewayIamBinding".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "condition".into(),
                    value: &condition_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "gateway".into(),
                    value: &gateway_binding.drop_type(),
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
                    name: "region".into(),
                    value: &region_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "role".into(),
                    value: &role_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        GatewayIamBindingResult {
            condition: o.get_field("condition"),
            etag: o.get_field("etag"),
            gateway: o.get_field("gateway"),
            members: o.get_field("members"),
            project: o.get_field("project"),
            region: o.get_field("region"),
            role: o.get_field("role"),
        }
    }
}
