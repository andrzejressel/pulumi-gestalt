/// Tenant configuration in a multi-tenant project.
///
/// You must enable the
/// [Google Identity Platform](https://console.cloud.google.com/marketplace/details/google-cloud-platform/customer-identity) in
/// the marketplace prior to using this resource.
///
/// You must [enable multi-tenancy](https://cloud.google.com/identity-platform/docs/multi-tenancy-quickstart) via
/// the Cloud Console prior to creating tenants.
///
///
///
/// ## Example Usage
///
/// ### Identity Platform Tenant Basic
///
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let tenant = tenant::create(
///         "tenant",
///         TenantArgs::builder()
///             .allow_password_signup(true)
///             .display_name("tenant")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Tenant can be imported using any of these accepted formats:
///
/// * `projects/{{project}}/tenants/{{name}}`
///
/// * `{{project}}/{{name}}`
///
/// * `{{name}}`
///
/// When using the `pulumi import` command, Tenant can be imported using one of the formats above. For example:
///
/// ```sh
/// $ pulumi import gcp:identityplatform/tenant:Tenant default projects/{{project}}/tenants/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:identityplatform/tenant:Tenant default {{project}}/{{name}}
/// ```
///
/// ```sh
/// $ pulumi import gcp:identityplatform/tenant:Tenant default {{name}}
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod tenant {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TenantArgs {
        /// Whether to allow email/password user authentication.
        #[builder(into, default)]
        pub allow_password_signup: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether authentication is disabled for the tenant. If true, the users under
        /// the disabled tenant are not allowed to sign-in. Admins of the disabled tenant
        /// are not able to manage its users.
        #[builder(into, default)]
        pub disable_auth: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Human friendly display name of the tenant.
        ///
        ///
        /// - - -
        #[builder(into)]
        pub display_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether to enable email link user authentication.
        #[builder(into, default)]
        pub enable_email_link_signin: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        #[builder(into, default)]
        pub project: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct TenantResult {
        /// Whether to allow email/password user authentication.
        pub allow_password_signup: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Whether authentication is disabled for the tenant. If true, the users under
        /// the disabled tenant are not allowed to sign-in. Admins of the disabled tenant
        /// are not able to manage its users.
        pub disable_auth: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Human friendly display name of the tenant.
        ///
        ///
        /// - - -
        pub display_name: pulumi_gestalt_rust::Output<String>,
        /// Whether to enable email link user authentication.
        pub enable_email_link_signin: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the tenant that is generated by the server
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The ID of the project in which the resource belongs.
        /// If it is not provided, the provider project is used.
        pub project: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TenantArgs,
    ) -> TenantResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let allow_password_signup_binding = args
            .allow_password_signup
            .get_output(context);
        let disable_auth_binding = args.disable_auth.get_output(context);
        let display_name_binding = args.display_name.get_output(context);
        let enable_email_link_signin_binding = args
            .enable_email_link_signin
            .get_output(context);
        let project_binding = args.project.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "gcp:identityplatform/tenant:Tenant".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "allowPasswordSignup".into(),
                    value: &allow_password_signup_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "disableAuth".into(),
                    value: &disable_auth_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "displayName".into(),
                    value: &display_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enableEmailLinkSignin".into(),
                    value: &enable_email_link_signin_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "project".into(),
                    value: &project_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        TenantResult {
            allow_password_signup: o.get_field("allowPasswordSignup"),
            disable_auth: o.get_field("disableAuth"),
            display_name: o.get_field("displayName"),
            enable_email_link_signin: o.get_field("enableEmailLinkSignin"),
            name: o.get_field("name"),
            project: o.get_field("project"),
        }
    }
}
