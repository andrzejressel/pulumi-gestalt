/// Provides a Cloudflare Authenticated Origin Pulls resource. A `cloudflare.AuthenticatedOriginPulls`
/// resource is required to use Per-Zone or Per-Hostname Authenticated
/// Origin Pulls.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let myAop = authenticated_origin_pulls::create(
///         "myAop",
///         AuthenticatedOriginPullsArgs::builder()
///             .enabled(true)
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
///     let myPerHostnameAop = authenticated_origin_pulls::create(
///         "myPerHostnameAop",
///         AuthenticatedOriginPullsArgs::builder()
///             .authenticated_origin_pulls_certificate("${myPerHostnameAopCert.id}")
///             .enabled(true)
///             .hostname("aop.example.com")
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
///     let myPerHostnameAopCert = authenticated_origin_pulls_certificate::create(
///         "myPerHostnameAopCert",
///         AuthenticatedOriginPullsCertificateArgs::builder()
///             .certificate("-----INSERT CERTIFICATE-----")
///             .private_key("-----INSERT PRIVATE KEY-----")
///             .type_("per-hostname")
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
///     let myPerZoneAop = authenticated_origin_pulls::create(
///         "myPerZoneAop",
///         AuthenticatedOriginPullsArgs::builder()
///             .authenticated_origin_pulls_certificate("${myPerZoneAopCert.id}")
///             .enabled(true)
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
///     let myPerZoneAopCert = authenticated_origin_pulls_certificate::create(
///         "myPerZoneAopCert",
///         AuthenticatedOriginPullsCertificateArgs::builder()
///             .certificate("-----INSERT CERTIFICATE-----")
///             .private_key("-----INSERT PRIVATE KEY-----")
///             .type_("per-zone")
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// global
///
/// ```sh
/// $ pulumi import cloudflare:index/authenticatedOriginPulls:AuthenticatedOriginPulls example <zone_id>
/// ```
///
/// per zone
///
/// ```sh
/// $ pulumi import cloudflare:index/authenticatedOriginPulls:AuthenticatedOriginPulls example <zone_id>/<certificate_id>
/// ```
///
/// per hostname
///
/// ```sh
/// $ pulumi import cloudflare:index/authenticatedOriginPulls:AuthenticatedOriginPulls example <zone_id>/<certificate_id>/<hostname>
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod authenticated_origin_pulls {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AuthenticatedOriginPullsArgs {
        /// The ID of an uploaded Authenticated Origin Pulls certificate. If no hostname is provided, this certificate will be used zone wide as Per-Zone Authenticated Origin Pulls.
        #[builder(into, default)]
        pub authenticated_origin_pulls_certificate: pulumi_gestalt_rust::Input<
            Option<String>,
        >,
        /// Whether to enable Authenticated Origin Pulls on the given zone or hostname.
        #[builder(into)]
        pub enabled: pulumi_gestalt_rust::Input<bool>,
        /// Specify a hostname to enable Per-Hostname Authenticated Origin Pulls on, using the provided certificate.
        #[builder(into, default)]
        pub hostname: pulumi_gestalt_rust::Input<Option<String>>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct AuthenticatedOriginPullsResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The ID of an uploaded Authenticated Origin Pulls certificate. If no hostname is provided, this certificate will be used zone wide as Per-Zone Authenticated Origin Pulls.
        pub authenticated_origin_pulls_certificate: pulumi_gestalt_rust::Output<
            Option<String>,
        >,
        /// Whether to enable Authenticated Origin Pulls on the given zone or hostname.
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        /// Specify a hostname to enable Per-Hostname Authenticated Origin Pulls on, using the provided certificate.
        pub hostname: pulumi_gestalt_rust::Output<Option<String>>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AuthenticatedOriginPullsArgs,
    ) -> AuthenticatedOriginPullsResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AuthenticatedOriginPullsArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> AuthenticatedOriginPullsResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AuthenticatedOriginPullsArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> AuthenticatedOriginPullsResult {
        let authenticated_origin_pulls_certificate_binding = args
            .authenticated_origin_pulls_certificate
            .get_output(ctx);
        let enabled_binding = args.enabled.get_output(ctx);
        let hostname_binding = args.hostname.get_output(ctx);
        let zone_id_binding = args.zone_id.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/authenticatedOriginPulls:AuthenticatedOriginPulls"
                .into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "authenticatedOriginPullsCertificate".into(),
                    value: &authenticated_origin_pulls_certificate_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hostname".into(),
                    value: &hostname_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        AuthenticatedOriginPullsResult {
            id: o.get_id(),
            urn: o.get_urn(),
            authenticated_origin_pulls_certificate: o
                .get_field("authenticatedOriginPullsCertificate"),
            enabled: o.get_field("enabled"),
            hostname: o.get_field("hostname"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
