/// Provides a Cloudflare Tunnel configuration resource.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   exampleTunnel:
///     type: cloudflare:ZeroTrustTunnelCloudflared
///     name: example_tunnel
///     properties:
///       accountId: f037e56e89293a057740de681ac9abbe
///       name: example_tunnel
///       secret: <32 character secret>
///   exampleConfig:
///     type: cloudflare:TunnelConfig
///     name: example_config
///     properties:
///       accountId: f037e56e89293a057740de681ac9abbe
///       tunnelId: ${exampleTunnel.id}
///       config:
///         warpRouting:
///           enabled: true
///         originRequest:
///           connectTimeout: 1m0s
///           tlsTimeout: 1m0s
///           tcpKeepAlive: 1m0s
///           noHappyEyeballs: false
///           keepAliveConnections: 1024
///           keepAliveTimeout: 1m0s
///           httpHostHeader: baz
///           originServerName: foobar
///           caPool: /path/to/unsigned/ca/pool
///           noTlsVerify: false
///           disableChunkedEncoding: false
///           bastionMode: false
///           proxyAddress: 10.0.0.1
///           proxyPort: '8123'
///           proxyType: socks
///           ipRules:
///             - prefix: /web
///               ports:
///                 - 80
///                 - 443
///               allow: false
///         ingressRules:
///           - hostname: foo
///             path: /bar
///             service: http://10.0.0.2:8080
///             originRequest:
///               connectTimeout: 2m0s
///               access:
///                 required: true
///                 teamName: terraform
///                 audTags:
///                   - AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
///           - service: https://10.0.0.3:8081
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/tunnelConfig:TunnelConfig example <account_id>/<tunnel_id>
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod tunnel_config {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TunnelConfigArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::Input<String>,
        /// Configuration block for Tunnel Configuration.
        #[builder(into)]
        pub config: pulumi_gestalt_rust::Input<super::types::TunnelConfigConfig>,
        /// Identifier of the Tunnel to target for this configuration.
        #[builder(into)]
        pub tunnel_id: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct TunnelConfigResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The account identifier to target for the resource.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// Configuration block for Tunnel Configuration.
        pub config: pulumi_gestalt_rust::Output<super::types::TunnelConfigConfig>,
        /// Identifier of the Tunnel to target for this configuration.
        pub tunnel_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TunnelConfigArgs,
    ) -> TunnelConfigResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TunnelConfigArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> TunnelConfigResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TunnelConfigArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> TunnelConfigResult {
        let account_id_binding = args.account_id.get_output(ctx);
        let config_binding = args.config.get_output(ctx);
        let tunnel_id_binding = args.tunnel_id.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/tunnelConfig:TunnelConfig".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "config".into(),
                    value: &config_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tunnelId".into(),
                    value: &tunnel_id_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        TunnelConfigResult {
            id: o.get_id(),
            urn: o.get_urn(),
            account_id: o.get_field("accountId"),
            config: o.get_field("config"),
            tunnel_id: o.get_field("tunnelId"),
        }
    }
}
