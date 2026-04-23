/// Resource for managing AWS Audit Manager Account Registration.
///
/// ## Example Usage
///
/// ### Basic Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = account_registration::create(
///         "example",
///         AccountRegistrationArgs::builder().build_struct(),
///     );
/// }
/// ```
///
/// ### Deregister On Destroy
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = account_registration::create(
///         "example",
///         AccountRegistrationArgs::builder().deregister_on_destroy(true).build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Audit Manager Account Registration resources using the `id`. For example:
///
/// ```sh
/// $ pulumi import aws:auditmanager/accountRegistration:AccountRegistration example us-east-1
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod account_registration {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountRegistrationArgs {
        /// Identifier for the delegated administrator account.
        #[builder(into, default)]
        pub delegated_admin_account: pulumi_gestalt_rust::Input<Option<String>>,
        /// Flag to deregister AuditManager in the account upon destruction. Defaults to `false` (ie. AuditManager will remain active in the account, even if this resource is removed).
        #[builder(into, default)]
        pub deregister_on_destroy: pulumi_gestalt_rust::Input<Option<bool>>,
        /// KMS key identifier.
        #[builder(into, default)]
        pub kms_key: pulumi_gestalt_rust::Input<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AccountRegistrationResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Identifier for the delegated administrator account.
        pub delegated_admin_account: pulumi_gestalt_rust::Output<Option<String>>,
        /// Flag to deregister AuditManager in the account upon destruction. Defaults to `false` (ie. AuditManager will remain active in the account, even if this resource is removed).
        pub deregister_on_destroy: pulumi_gestalt_rust::Output<Option<bool>>,
        /// KMS key identifier.
        pub kms_key: pulumi_gestalt_rust::Output<Option<String>>,
        /// Status of the account registration request.
        pub status: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AccountRegistrationArgs,
    ) -> AccountRegistrationResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AccountRegistrationArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> AccountRegistrationResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AccountRegistrationArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> AccountRegistrationResult {
        let delegated_admin_account_binding = args
            .delegated_admin_account
            .get_output(ctx);
        let deregister_on_destroy_binding = args.deregister_on_destroy.get_output(ctx);
        let kms_key_binding = args.kms_key.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:auditmanager/accountRegistration:AccountRegistration".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "delegatedAdminAccount".into(),
                    value: &delegated_admin_account_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "deregisterOnDestroy".into(),
                    value: &deregister_on_destroy_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kmsKey".into(),
                    value: &kms_key_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        AccountRegistrationResult {
            id: o.get_id(),
            urn: o.get_urn(),
            delegated_admin_account: o.get_field("delegatedAdminAccount"),
            deregister_on_destroy: o.get_field("deregisterOnDestroy"),
            kms_key: o.get_field("kmsKey"),
            status: o.get_field("status"),
        }
    }
}
