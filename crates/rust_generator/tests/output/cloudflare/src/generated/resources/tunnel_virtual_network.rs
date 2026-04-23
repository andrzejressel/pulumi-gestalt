/// Provides a resource, that manages Cloudflare tunnel virtual networks
/// for Zero Trust. Tunnel virtual networks are used for segregation of
/// Tunnel IP Routes via Virtualized Networks to handle overlapping
/// private IPs in your origins.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = tunnel_virtual_network::create(
///         "example",
///         TunnelVirtualNetworkArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .comment("New tunnel virtual network for documentation")
///             .name("vnet-for-documentation")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/tunnelVirtualNetwork:TunnelVirtualNetwork example <account_id>/<vnet_id>
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod tunnel_virtual_network {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TunnelVirtualNetworkArgs {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::Input<String>,
        /// Description of the tunnel virtual network.
        #[builder(into, default)]
        pub comment: pulumi_gestalt_rust::Input<Option<String>>,
        /// Whether this virtual network is the default one for the account. This means IP Routes belong to this virtual network and Teams Clients in the account route through this virtual network, unless specified otherwise for each case.
        #[builder(into, default)]
        pub is_default_network: pulumi_gestalt_rust::Input<Option<bool>>,
        /// A user-friendly name chosen when the virtual network is created.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct TunnelVirtualNetworkResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// Description of the tunnel virtual network.
        pub comment: pulumi_gestalt_rust::Output<Option<String>>,
        /// Whether this virtual network is the default one for the account. This means IP Routes belong to this virtual network and Teams Clients in the account route through this virtual network, unless specified otherwise for each case.
        pub is_default_network: pulumi_gestalt_rust::Output<Option<bool>>,
        /// A user-friendly name chosen when the virtual network is created.
        pub name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TunnelVirtualNetworkArgs,
    ) -> TunnelVirtualNetworkResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TunnelVirtualNetworkArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> TunnelVirtualNetworkResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TunnelVirtualNetworkArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> TunnelVirtualNetworkResult {
        let account_id_binding = args.account_id.get_output(ctx);
        let comment_binding = args.comment.get_output(ctx);
        let is_default_network_binding = args.is_default_network.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/tunnelVirtualNetwork:TunnelVirtualNetwork".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "comment".into(),
                    value: &comment_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "isDefaultNetwork".into(),
                    value: &is_default_network_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        TunnelVirtualNetworkResult {
            id: o.get_id(),
            urn: o.get_urn(),
            account_id: o.get_field("accountId"),
            comment: o.get_field("comment"),
            is_default_network: o.get_field("isDefaultNetwork"),
            name: o.get_field("name"),
        }
    }
}
