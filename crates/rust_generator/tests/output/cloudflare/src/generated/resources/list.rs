/// ## Example Usage
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/list:List example <account_id>/<list_id>
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod list {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ListArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::Input<String>,
        /// An optional description of the list.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::Input<Option<String>>,
        /// The items in the list.
        #[builder(into, default)]
        pub items: pulumi_gestalt_rust::Input<
            Option<Vec<super::types::ListItem>>,
        >,
        /// The type of items the list will contain. Must provide only one of: `ip`, `redirect`, `hostname`, `asn`..
        #[builder(into)]
        pub kind: pulumi_gestalt_rust::Input<String>,
        /// The name of the list.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct ListResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The account identifier to target for the resource.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// An optional description of the list.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The items in the list.
        pub items: pulumi_gestalt_rust::Output<Option<Vec<super::types::ListItem>>>,
        /// The type of items the list will contain. Must provide only one of: `ip`, `redirect`, `hostname`, `asn`..
        pub kind: pulumi_gestalt_rust::Output<String>,
        /// The name of the list.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ListArgs,
    ) -> ListResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ListArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ListResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ListArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ListResult {
        let account_id_binding = args.account_id.get_output(ctx);
        let description_binding = args.description.get_output(ctx);
        let items_binding = args.items.get_output(ctx);
        let kind_binding = args.kind.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/list:List".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "items".into(),
                    value: &items_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kind".into(),
                    value: &kind_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        ListResult {
            id: o.get_id(),
            urn: o.get_urn(),
            account_id: o.get_field("accountId"),
            description: o.get_field("description"),
            items: o.get_field("items"),
            kind: o.get_field("kind"),
            name: o.get_field("name"),
        }
    }
}
