/// Provides a resource, that manages Keyless certificates.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = keyless_certificate::create(
///         "example",
///         KeylessCertificateArgs::builder()
///             .bundle_method("ubiquitous")
///             .certificate("-----INSERT CERTIFICATE-----")
///             .enabled(true)
///             .host("example.com")
///             .name("example.com Keyless SSL")
///             .port(24008)
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/keylessCertificate:KeylessCertificate example <zone_id>/<keyless_certificate_id>
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod keyless_certificate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct KeylessCertificateArgs {
        /// A ubiquitous bundle has the highest probability of being verified everywhere, even by clients using outdated or unusual trust stores. An optimal bundle uses the shortest chain and newest intermediates. And the force bundle verifies the chain, but does not otherwise modify it. Available values: `ubiquitous`, `optimal`, `force`. Defaults to `ubiquitous`. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub bundle_method: pulumi_gestalt_rust::Input<Option<String>>,
        /// The zone's SSL certificate or SSL certificate and intermediate(s). **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub certificate: pulumi_gestalt_rust::Input<String>,
        /// Whether the KeyLess SSL is on.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::Input<Option<bool>>,
        /// The KeyLess SSL host.
        #[builder(into)]
        pub host: pulumi_gestalt_rust::Input<String>,
        /// The KeyLess SSL name.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::Input<Option<String>>,
        /// The KeyLess SSL port used to communicate between Cloudflare and the client's KeyLess SSL server. Defaults to `24008`.
        #[builder(into, default)]
        pub port: pulumi_gestalt_rust::Input<Option<i32>>,
        /// The zone identifier to target for the resource.
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct KeylessCertificateResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// A ubiquitous bundle has the highest probability of being verified everywhere, even by clients using outdated or unusual trust stores. An optimal bundle uses the shortest chain and newest intermediates. And the force bundle verifies the chain, but does not otherwise modify it. Available values: `ubiquitous`, `optimal`, `force`. Defaults to `ubiquitous`. **Modifying this attribute will force creation of a new resource.**
        pub bundle_method: pulumi_gestalt_rust::Output<Option<String>>,
        /// The zone's SSL certificate or SSL certificate and intermediate(s). **Modifying this attribute will force creation of a new resource.**
        pub certificate: pulumi_gestalt_rust::Output<String>,
        /// Whether the KeyLess SSL is on.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The KeyLess SSL host.
        pub host: pulumi_gestalt_rust::Output<String>,
        /// The KeyLess SSL name.
        pub name: pulumi_gestalt_rust::Output<Option<String>>,
        /// The KeyLess SSL port used to communicate between Cloudflare and the client's KeyLess SSL server. Defaults to `24008`.
        pub port: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Status of the KeyLess SSL.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// The zone identifier to target for the resource.
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: KeylessCertificateArgs,
    ) -> KeylessCertificateResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: KeylessCertificateArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> KeylessCertificateResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: KeylessCertificateArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> KeylessCertificateResult {
        let bundle_method_binding = args.bundle_method.get_output(ctx);
        let certificate_binding = args.certificate.get_output(ctx);
        let enabled_binding = args.enabled.get_output(ctx);
        let host_binding = args.host.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let port_binding = args.port.get_output(ctx);
        let zone_id_binding = args.zone_id.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/keylessCertificate:KeylessCertificate".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bundleMethod".into(),
                    value: &bundle_method_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificate".into(),
                    value: &certificate_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "host".into(),
                    value: &host_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "port".into(),
                    value: &port_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        KeylessCertificateResult {
            id: o.get_id(),
            urn: o.get_urn(),
            bundle_method: o.get_field("bundleMethod"),
            certificate: o.get_field("certificate"),
            enabled: o.get_field("enabled"),
            host: o.get_field("host"),
            name: o.get_field("name"),
            port: o.get_field("port"),
            status: o.get_field("status"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
