/// Provides a Cloudflare Split Tunnel resource. Split tunnels are used to either
/// include or exclude lists of routes from the WARP client's tunnel.
///
/// ## Import
///
/// Split Tunnels for default device policies must use "default" as the policy ID.
///
/// ```sh
/// $ pulumi import cloudflare:index/splitTunnel:SplitTunnel example <account_id>/<policy_id>/<mode>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod split_tunnel {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SplitTunnelArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The mode of the split tunnel policy. Available values: `include`, `exclude`.
        #[builder(into)]
        pub mode: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The settings policy for which to configure this split tunnel policy.
        #[builder(into, default)]
        pub policy_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The value of the tunnel attributes.
        #[builder(into)]
        pub tunnels: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::types::SplitTunnelTunnel>,
        >,
    }
    #[allow(dead_code)]
    pub struct SplitTunnelResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// The mode of the split tunnel policy. Available values: `include`, `exclude`.
        pub mode: pulumi_gestalt_rust::Output<String>,
        /// The settings policy for which to configure this split tunnel policy.
        pub policy_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// The value of the tunnel attributes.
        pub tunnels: pulumi_gestalt_rust::Output<Vec<super::types::SplitTunnelTunnel>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: SplitTunnelArgs,
    ) -> SplitTunnelResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let mode_binding = args.mode.get_output(context);
        let policy_id_binding = args.policy_id.get_output(context);
        let tunnels_binding = args.tunnels.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/splitTunnel:SplitTunnel".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mode".into(),
                    value: &mode_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "policyId".into(),
                    value: &policy_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tunnels".into(),
                    value: &tunnels_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        SplitTunnelResult {
            account_id: o.get_field("accountId"),
            mode: o.get_field("mode"),
            policy_id: o.get_field("policyId"),
            tunnels: o.get_field("tunnels"),
        }
    }
}
