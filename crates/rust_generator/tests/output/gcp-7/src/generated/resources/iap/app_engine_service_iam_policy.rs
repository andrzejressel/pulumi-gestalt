/// Three different resources help you manage your IAM policy for Identity-Aware Proxy AppEngineService. Each of these resources serves a different use case:
///
/// * `gcp.iap.AppEngineServiceIamPolicy`: Authoritative. Sets the IAM policy for the appengineservice and replaces any existing policy already attached.
/// * `gcp.iap.AppEngineServiceIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the appengineservice are preserved.
/// * `gcp.iap.AppEngineServiceIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the appengineservice are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.iap.AppEngineServiceIamPolicy`: Retrieves the IAM policy for the appengineservice
///
/// > **Note:** `gcp.iap.AppEngineServiceIamPolicy` **cannot** be used in conjunction with `gcp.iap.AppEngineServiceIamBinding` and `gcp.iap.AppEngineServiceIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.iap.AppEngineServiceIamBinding` resources **can be** used in conjunction with `gcp.iap.AppEngineServiceIamMember` resources **only if** they do not grant privilege to the same role.
///
/// > **Note:**  This resource supports IAM Conditions but they have some known limitations which can be found [here](https://cloud.google.com/iam/docs/conditions-overview#limitations). Please review this article if you are having issues with IAM Conditions.
///
///
/// ## gcp.iap.AppEngineServiceIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:iap:AppEngineServiceIamPolicy
///     properties:
///       project: ${version.project}
///       appId: ${version.project}
///       service: ${version.service}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/iap.httpsResourceAccessor
///             members:
///               - user:jane@example.com
/// ```
///
/// With IAM Conditions:
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:iap:AppEngineServiceIamPolicy
///     properties:
///       project: ${version.project}
///       appId: ${version.project}
///       service: ${version.service}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/iap.httpsResourceAccessor
///             members:
///               - user:jane@example.com
///             condition:
///               title: expires_after_2019_12_31
///               description: Expiring at midnight of 2019-12-31
///               expression: request.time < timestamp("2020-01-01T00:00:00Z")
/// ```
/// ## gcp.iap.AppEngineServiceIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = app_engine_service_iam_binding::create(
///         "binding",
///         AppEngineServiceIamBindingArgs::builder()
///             .app_id("${version.project}")
///             .members(vec!["user:jane@example.com",])
///             .project("${version.project}")
///             .role("roles/iap.httpsResourceAccessor")
///             .service("${version.service}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// With IAM Conditions:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = app_engine_service_iam_binding::create(
///         "binding",
///         AppEngineServiceIamBindingArgs::builder()
///             .app_id("${version.project}")
///             .condition(
///                 AppEngineServiceIamBindingCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .members(vec!["user:jane@example.com",])
///             .project("${version.project}")
///             .role("roles/iap.httpsResourceAccessor")
///             .service("${version.service}")
///             .build_struct(),
///     );
/// }
/// ```
/// ## gcp.iap.AppEngineServiceIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = app_engine_service_iam_member::create(
///         "member",
///         AppEngineServiceIamMemberArgs::builder()
///             .app_id("${version.project}")
///             .member("user:jane@example.com")
///             .project("${version.project}")
///             .role("roles/iap.httpsResourceAccessor")
///             .service("${version.service}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// With IAM Conditions:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = app_engine_service_iam_member::create(
///         "member",
///         AppEngineServiceIamMemberArgs::builder()
///             .app_id("${version.project}")
///             .condition(
///                 AppEngineServiceIamMemberCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .member("user:jane@example.com")
///             .project("${version.project}")
///             .role("roles/iap.httpsResourceAccessor")
///             .service("${version.service}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## This resource supports User Project Overrides.
///
/// -
///
/// # IAM policy for Identity-Aware Proxy AppEngineService
/// Three different resources help you manage your IAM policy for Identity-Aware Proxy AppEngineService. Each of these resources serves a different use case:
///
/// * `gcp.iap.AppEngineServiceIamPolicy`: Authoritative. Sets the IAM policy for the appengineservice and replaces any existing policy already attached.
/// * `gcp.iap.AppEngineServiceIamBinding`: Authoritative for a given role. Updates the IAM policy to grant a role to a list of members. Other roles within the IAM policy for the appengineservice are preserved.
/// * `gcp.iap.AppEngineServiceIamMember`: Non-authoritative. Updates the IAM policy to grant a role to a new member. Other members for the role for the appengineservice are preserved.
///
/// A data source can be used to retrieve policy data in advent you do not need creation
///
/// * `gcp.iap.AppEngineServiceIamPolicy`: Retrieves the IAM policy for the appengineservice
///
/// > **Note:** `gcp.iap.AppEngineServiceIamPolicy` **cannot** be used in conjunction with `gcp.iap.AppEngineServiceIamBinding` and `gcp.iap.AppEngineServiceIamMember` or they will fight over what your policy should be.
///
/// > **Note:** `gcp.iap.AppEngineServiceIamBinding` resources **can be** used in conjunction with `gcp.iap.AppEngineServiceIamMember` resources **only if** they do not grant privilege to the same role.
///
/// > **Note:**  This resource supports IAM Conditions but they have some known limitations which can be found [here](https://cloud.google.com/iam/docs/conditions-overview#limitations). Please review this article if you are having issues with IAM Conditions.
///
///
/// ## gcp.iap.AppEngineServiceIamPolicy
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:iap:AppEngineServiceIamPolicy
///     properties:
///       project: ${version.project}
///       appId: ${version.project}
///       service: ${version.service}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/iap.httpsResourceAccessor
///             members:
///               - user:jane@example.com
/// ```
///
/// With IAM Conditions:
///
/// ```yaml
/// resources:
///   policy:
///     type: gcp:iap:AppEngineServiceIamPolicy
///     properties:
///       project: ${version.project}
///       appId: ${version.project}
///       service: ${version.service}
///       policyData: ${admin.policyData}
/// variables:
///   admin:
///     fn::invoke:
///       function: gcp:organizations:getIAMPolicy
///       arguments:
///         bindings:
///           - role: roles/iap.httpsResourceAccessor
///             members:
///               - user:jane@example.com
///             condition:
///               title: expires_after_2019_12_31
///               description: Expiring at midnight of 2019-12-31
///               expression: request.time < timestamp("2020-01-01T00:00:00Z")
/// ```
/// ## gcp.iap.AppEngineServiceIamBinding
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = app_engine_service_iam_binding::create(
///         "binding",
///         AppEngineServiceIamBindingArgs::builder()
///             .app_id("${version.project}")
///             .members(vec!["user:jane@example.com",])
///             .project("${version.project}")
///             .role("roles/iap.httpsResourceAccessor")
///             .service("${version.service}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// With IAM Conditions:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let binding = app_engine_service_iam_binding::create(
///         "binding",
///         AppEngineServiceIamBindingArgs::builder()
///             .app_id("${version.project}")
///             .condition(
///                 AppEngineServiceIamBindingCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .members(vec!["user:jane@example.com",])
///             .project("${version.project}")
///             .role("roles/iap.httpsResourceAccessor")
///             .service("${version.service}")
///             .build_struct(),
///     );
/// }
/// ```
/// ## gcp.iap.AppEngineServiceIamMember
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = app_engine_service_iam_member::create(
///         "member",
///         AppEngineServiceIamMemberArgs::builder()
///             .app_id("${version.project}")
///             .member("user:jane@example.com")
///             .project("${version.project}")
///             .role("roles/iap.httpsResourceAccessor")
///             .service("${version.service}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// With IAM Conditions:
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let member = app_engine_service_iam_member::create(
///         "member",
///         AppEngineServiceIamMemberArgs::builder()
///             .app_id("${version.project}")
///             .condition(
///                 AppEngineServiceIamMemberCondition::builder()
///                     .description("Expiring at midnight of 2019-12-31")
///                     .expression("request.time < timestamp(\"2020-01-01T00:00:00Z\")")
///                     .title("expires_after_2019_12_31")
///                     .build_struct(),
///             )
///             .member("user:jane@example.com")
///             .project("${version.project}")
///             .role("roles/iap.httpsResourceAccessor")
///             .service("${version.service}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// For all import syntaxes, the "resource in question" can take any of the following forms:
///
/// * projects/{{project}}/iap_web/appengine-{{appId}}/services/{{service}}
///
/// * {{project}}/{{appId}}/{{service}}
///
/// * {{appId}}/{{service}}
///
/// * {{service}}
///
/// Any variables not passed in the import command will be taken from the provider configuration.
///
/// Identity-Aware Proxy appengineservice IAM resources can be imported using the resource identifiers, role, and member.
///
/// IAM member imports use space-delimited identifiers: the resource in question, the role, and the member identity, e.g.
///
/// ```sh
/// $ pulumi import gcp:iap/appEngineServiceIamPolicy:AppEngineServiceIamPolicy editor "projects/{{project}}/iap_web/appengine-{{appId}}/services/{{service}} roles/iap.httpsResourceAccessor user:jane@example.com"
/// ```
///
/// IAM binding imports use space-delimited identifiers: the resource in question and the role, e.g.
///
/// ```sh
/// $ pulumi import gcp:iap/appEngineServiceIamPolicy:AppEngineServiceIamPolicy editor "projects/{{project}}/iap_web/appengine-{{appId}}/services/{{service}} roles/iap.httpsResourceAccessor"
/// ```
///
/// IAM policy imports use the identifier of the resource in question, e.g.
///
/// ```sh
/// $ pulumi import gcp:iap/appEngineServiceIamPolicy:AppEngineServiceIamPolicy editor projects/{{project}}/iap_web/appengine-{{appId}}/services/{{service}}
/// ```
///
/// -> **Custom Roles** If you're importing a IAM resource with a custom role, make sure to use the
///
///  full name of the custom role, e.g. `[projects/my-project|organizations/my-org]/roles/my-custom-role`.
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod app_engine_service_iam_policy {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AppEngineServiceIamPolicyArgs {
        /// Id of the App Engine application. Used to find the parent resource to bind the IAM policy to
        #[builder(into)]
        pub app_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        #[builder(into)]
        pub policy_data: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Service id of the App Engine application Used to find the parent resource to bind the IAM policy to
        #[builder(into)]
        pub service: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct AppEngineServiceIamPolicyResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Id of the App Engine application. Used to find the parent resource to bind the IAM policy to
        pub app_id: pulumi_gestalt_rust::Output<String>,
        /// (Computed) The etag of the IAM policy.
        pub etag: pulumi_gestalt_rust::Output<String>,
        /// The policy data generated by
        /// a `gcp.organizations.getIAMPolicy` data source.
        pub policy_data: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the project will be parsed from the identifier of the parent resource. If no project is provided in the parent identifier and no project is specified, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
        /// Service id of the App Engine application Used to find the parent resource to bind the IAM policy to
        pub service: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AppEngineServiceIamPolicyArgs,
    ) -> AppEngineServiceIamPolicyResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let app_id_binding = args.app_id.get_output(context);
        let policy_data_binding = args.policy_data.get_output(context);
        let project_binding = args.project.get_output(context);
        let service_binding = args.service.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:iap/appEngineServiceIamPolicy:AppEngineServiceIamPolicy".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "appId".into(),
                    value: &app_id_binding.drop_type(),
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
                    name: "service".into(),
                    value: &service_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        AppEngineServiceIamPolicyResult {
            id: o.get_field("id"),
            app_id: o.get_field("appId"),
            etag: o.get_field("etag"),
            policy_data: o.get_field("policyData"),
            project: o.get_field("project"),
            service: o.get_field("service"),
        }
    }
}
