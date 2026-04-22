/// Manages a Logic App Integration Account Partner.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleIntegrationAccount = integration_account::create(
///         "exampleIntegrationAccount",
///         IntegrationAccountArgs::builder()
///             .location("${example.location}")
///             .name("example-ia")
///             .resource_group_name("${example.name}")
///             .sku_name("Standard")
///             .build_struct(),
///     );
///     let exampleIntegrationAccountPartner = integration_account_partner::create(
///         "exampleIntegrationAccountPartner",
///         IntegrationAccountPartnerArgs::builder()
///             .business_identities(
///                 vec![
///                     IntegrationAccountPartnerBusinessIdentity::builder().qualifier("ZZ")
///                     .value("AA").build_struct(),
///                 ],
///             )
///             .integration_account_name("${exampleIntegrationAccount.name}")
///             .name("example-iap")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Logic App Integration Account Partners can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:logicapps/integrationAccountPartner:IntegrationAccountPartner example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Logic/integrationAccounts/account1/partners/partner1
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod integration_account_partner {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct IntegrationAccountPartnerArgs {
        /// A `business_identity` block as documented below.
        #[builder(into)]
        pub business_identities: pulumi_gestalt_rust::Input<
            Vec<
                super::super::types::logicapps::IntegrationAccountPartnerBusinessIdentity,
            >,
        >,
        /// The name of the Logic App Integration Account. Changing this forces a new Logic App Integration Account Partner to be created.
        #[builder(into)]
        pub integration_account_name: pulumi_gestalt_rust::Input<String>,
        /// A JSON mapping of any Metadata for this Logic App Integration Account Partner.
        #[builder(into, default)]
        pub metadata: pulumi_gestalt_rust::Input<Option<String>>,
        /// The name which should be used for this Logic App Integration Account Partner. Changing this forces a new Logic App Integration Account Partner to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// The name of the Resource Group where the Logic App Integration Account Partner should exist. Changing this forces a new Logic App Integration Account Partner to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct IntegrationAccountPartnerResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// A `business_identity` block as documented below.
        pub business_identities: pulumi_gestalt_rust::Output<
            Vec<
                super::super::types::logicapps::IntegrationAccountPartnerBusinessIdentity,
            >,
        >,
        /// The name of the Logic App Integration Account. Changing this forces a new Logic App Integration Account Partner to be created.
        pub integration_account_name: pulumi_gestalt_rust::Output<String>,
        /// A JSON mapping of any Metadata for this Logic App Integration Account Partner.
        pub metadata: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name which should be used for this Logic App Integration Account Partner. Changing this forces a new Logic App Integration Account Partner to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Logic App Integration Account Partner should exist. Changing this forces a new Logic App Integration Account Partner to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IntegrationAccountPartnerArgs,
    ) -> IntegrationAccountPartnerResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IntegrationAccountPartnerArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> IntegrationAccountPartnerResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: IntegrationAccountPartnerArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> IntegrationAccountPartnerResult {
        let business_identities_binding = args.business_identities.get_output(ctx);
        let integration_account_name_binding = args
            .integration_account_name
            .get_output(ctx);
        let metadata_binding = args.metadata.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let resource_group_name_binding = args.resource_group_name.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:logicapps/integrationAccountPartner:IntegrationAccountPartner"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "businessIdentities".into(),
                    value: &business_identities_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "integrationAccountName".into(),
                    value: &integration_account_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "metadata".into(),
                    value: &metadata_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        IntegrationAccountPartnerResult {
            id: o.get_id(),
            urn: o.get_urn(),
            business_identities: o.get_field("businessIdentities"),
            integration_account_name: o.get_field("integrationAccountName"),
            metadata: o.get_field("metadata"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
        }
    }
}
