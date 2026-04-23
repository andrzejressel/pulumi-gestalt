/// Resource for managing an AWS Verified Permissions Policy Template.
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
///     let example = policy_template::create(
///         "example",
///         PolicyTemplateArgs::builder()
///             .policy_store_id("${exampleAwsVerifiedpermissionsPolicyStore.id}")
///             .statement(
///                 "permit (principal in ?principal, action in PhotoFlash::Action::\"FullPhotoAccess\", resource == ?resource) unless { resource.IsPrivate };",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import Verified Permissions Policy Store using the `policy_store_id:policy_template_id`. For example:
///
/// ```sh
/// $ pulumi import aws:verifiedpermissions/policyTemplate:PolicyTemplate example policyStoreId:policyTemplateId
/// ```
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod policy_template {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PolicyTemplateArgs {
        /// Provides a description for the policy template.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::Input<Option<String>>,
        /// The ID of the Policy Store.
        #[builder(into)]
        pub policy_store_id: pulumi_gestalt_rust::Input<String>,
        /// Defines the content of the statement, written in Cedar policy language.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub statement: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct PolicyTemplateResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The date the Policy Store was created.
        pub created_date: pulumi_gestalt_rust::Output<String>,
        /// Provides a description for the policy template.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Policy Store.
        pub policy_store_id: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Policy Store.
        pub policy_template_id: pulumi_gestalt_rust::Output<String>,
        /// Defines the content of the statement, written in Cedar policy language.
        ///
        /// The following arguments are optional:
        pub statement: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PolicyTemplateArgs,
    ) -> PolicyTemplateResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PolicyTemplateArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> PolicyTemplateResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PolicyTemplateArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> PolicyTemplateResult {
        let description_binding = args.description.get_output(ctx);
        let policy_store_id_binding = args.policy_store_id.get_output(ctx);
        let statement_binding = args.statement.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:verifiedpermissions/policyTemplate:PolicyTemplate".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyStoreId".into(),
                    value: &policy_store_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "statement".into(),
                    value: &statement_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        PolicyTemplateResult {
            id: o.get_id(),
            urn: o.get_urn(),
            created_date: o.get_field("createdDate"),
            description: o.get_field("description"),
            policy_store_id: o.get_field("policyStoreId"),
            policy_template_id: o.get_field("policyTemplateId"),
            statement: o.get_field("statement"),
        }
    }
}
