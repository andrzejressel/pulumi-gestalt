/// Provides individual list items (IPs, Redirects, ASNs, Hostnames) to be used in Edge Rules Engine
/// across all zones within the same account.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let exampleAsnItem = list_item::create(
///         "exampleAsnItem",
///         ListItemArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .asn(6789)
///             .comment("List Item Comment")
///             .list_id("${exampleAsnList.id}")
///             .build_struct(),
///     );
///     let exampleAsnList = list::create(
///         "exampleAsnList",
///         ListArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .description("example ASNs for a list")
///             .kind("asn")
///             .name("example_asn_list")
///             .build_struct(),
///     );
///     let exampleHostnameItem = list_item::create(
///         "exampleHostnameItem",
///         ListItemArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .comment("List Item Comment")
///             .hostname(
///                 ListItemHostname::builder().urlHostname("example.com").build_struct(),
///             )
///             .list_id("${exampleHostnameList.id}")
///             .build_struct(),
///     );
///     let exampleHostnameList = list::create(
///         "exampleHostnameList",
///         ListArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .description("example Hostnames for a list")
///             .kind("hostname")
///             .name("example_hostname_list")
///             .build_struct(),
///     );
///     let exampleIpItem = list_item::create(
///         "exampleIpItem",
///         ListItemArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .comment("List Item Comment")
///             .ip("192.0.2.0")
///             .list_id("${exampleIpList.id}")
///             .build_struct(),
///     );
///     let exampleIpList = list::create(
///         "exampleIpList",
///         ListArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .description("example IPs for a list")
///             .kind("ip")
///             .name("example_list")
///             .build_struct(),
///     );
///     let exampleRedirectItem = list_item::create(
///         "exampleRedirectItem",
///         ListItemArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .list_id("${exampleIpList.id}")
///             .redirect(
///                 ListItemRedirect::builder()
///                     .sourceUrl("https://source.tld/")
///                     .statusCode(302)
///                     .subpathMatching(true)
///                     .targetUrl("https://target.tld")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
///     let exampleRedirectList = list::create(
///         "exampleRedirectList",
///         ListArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .description("example Redirects for a list")
///             .kind("redirect")
///             .name("example_list")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/listItem:ListItem example <account_id>/<list_id>/<item_id>
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod list_item {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ListItemArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::Input<String>,
        /// Autonomous system number to include in the list. Must provide only one of: `ip`, `asn`, `redirect`, `hostname`.
        #[builder(into, default)]
        pub asn: pulumi_gestalt_rust::Input<Option<i32>>,
        /// An optional comment for the item.
        #[builder(into, default)]
        pub comment: pulumi_gestalt_rust::Input<Option<String>>,
        /// Hostname to store in the list. Must provide only one of: `ip`, `asn`, `redirect`, `hostname`.
        #[builder(into, default)]
        pub hostname: pulumi_gestalt_rust::Input<
            Option<super::types::ListItemHostname>,
        >,
        /// IP address to include in the list. Must provide only one of: `ip`, `asn`, `redirect`, `hostname`.
        #[builder(into, default)]
        pub ip: pulumi_gestalt_rust::Input<Option<String>>,
        /// The list identifier to target for the resource.
        #[builder(into)]
        pub list_id: pulumi_gestalt_rust::Input<String>,
        /// Redirect configuration to store in the list. Must provide only one of: `ip`, `asn`, `redirect`, `hostname`.
        #[builder(into, default)]
        pub redirect: pulumi_gestalt_rust::Input<
            Option<super::types::ListItemRedirect>,
        >,
    }
    #[allow(dead_code)]
    pub struct ListItemResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The account identifier to target for the resource.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// Autonomous system number to include in the list. Must provide only one of: `ip`, `asn`, `redirect`, `hostname`.
        pub asn: pulumi_gestalt_rust::Output<Option<i32>>,
        /// An optional comment for the item.
        pub comment: pulumi_gestalt_rust::Output<Option<String>>,
        /// Hostname to store in the list. Must provide only one of: `ip`, `asn`, `redirect`, `hostname`.
        pub hostname: pulumi_gestalt_rust::Output<
            Option<super::types::ListItemHostname>,
        >,
        /// IP address to include in the list. Must provide only one of: `ip`, `asn`, `redirect`, `hostname`.
        pub ip: pulumi_gestalt_rust::Output<Option<String>>,
        /// The list identifier to target for the resource.
        pub list_id: pulumi_gestalt_rust::Output<String>,
        /// Redirect configuration to store in the list. Must provide only one of: `ip`, `asn`, `redirect`, `hostname`.
        pub redirect: pulumi_gestalt_rust::Output<
            Option<super::types::ListItemRedirect>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ListItemArgs,
    ) -> ListItemResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ListItemArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ListItemResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ListItemArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ListItemResult {
        let account_id_binding = args.account_id.get_output(ctx);
        let asn_binding = args.asn.get_output(ctx);
        let comment_binding = args.comment.get_output(ctx);
        let hostname_binding = args.hostname.get_output(ctx);
        let ip_binding = args.ip.get_output(ctx);
        let list_id_binding = args.list_id.get_output(ctx);
        let redirect_binding = args.redirect.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/listItem:ListItem".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "asn".into(),
                    value: &asn_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "comment".into(),
                    value: &comment_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hostname".into(),
                    value: &hostname_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ip".into(),
                    value: &ip_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "listId".into(),
                    value: &list_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "redirect".into(),
                    value: &redirect_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        ListItemResult {
            id: o.get_id(),
            urn: o.get_urn(),
            account_id: o.get_field("accountId"),
            asn: o.get_field("asn"),
            comment: o.get_field("comment"),
            hostname: o.get_field("hostname"),
            ip: o.get_field("ip"),
            list_id: o.get_field("listId"),
            redirect: o.get_field("redirect"),
        }
    }
}
