/// Manages a Cognitive Account Rai Blocklist.
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
///             .location("Brazil South")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleAccount = account::create(
///         "exampleAccount",
///         AccountArgs::builder()
///             .kind("OpenAI")
///             .location("${example.location}")
///             .name("example-ca")
///             .resource_group_name("${example.name}")
///             .sku_name("S0")
///             .build_struct(),
///     );
///     let exampleAccountRaiBlocklist = account_rai_blocklist::create(
///         "exampleAccountRaiBlocklist",
///         AccountRaiBlocklistArgs::builder()
///             .cognitive_account_id("${exampleAccount.id}")
///             .description("Azure OpenAI Rai Blocklist")
///             .name("example-crb")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Cognitive Account Rai Blocklist can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:cognitive/accountRaiBlocklist:AccountRaiBlocklist example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.CognitiveServices/accounts/account1/raiBlocklists/raiblocklist1
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod account_rai_blocklist {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccountRaiBlocklistArgs {
        /// The ID of the Cognitive Services Account. Changing this forces a new Cognitive Account Rai Blocklist to be created.
        #[builder(into)]
        pub cognitive_account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A short description for the Cognitive Account Rai Blocklist.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Cognitive Account Rai Blocklist. Changing this forces a new Cognitive Account Rai Blocklist to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AccountRaiBlocklistResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The ID of the Cognitive Services Account. Changing this forces a new Cognitive Account Rai Blocklist to be created.
        pub cognitive_account_id: pulumi_gestalt_rust::Output<String>,
        /// A short description for the Cognitive Account Rai Blocklist.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the Cognitive Account Rai Blocklist. Changing this forces a new Cognitive Account Rai Blocklist to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AccountRaiBlocklistArgs,
    ) -> AccountRaiBlocklistResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AccountRaiBlocklistArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> AccountRaiBlocklistResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AccountRaiBlocklistArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> AccountRaiBlocklistResult {
        let cognitive_account_id_binding = args.cognitive_account_id.get_output(ctx);
        let description_binding = args.description.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:cognitive/accountRaiBlocklist:AccountRaiBlocklist".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "cognitiveAccountId".into(),
                    value: &cognitive_account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        AccountRaiBlocklistResult {
            id: o.get_id(),
            urn: o.get_urn(),
            cognitive_account_id: o.get_field("cognitiveAccountId"),
            description: o.get_field("description"),
            name: o.get_field("name"),
        }
    }
}
