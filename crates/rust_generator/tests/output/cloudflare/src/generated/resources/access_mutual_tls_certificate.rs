/// Provides a Cloudflare Access Mutual TLS Certificate resource.
/// Mutual TLS authentication ensures that the traffic is secure and
/// trusted in both directions between a client and server and can be
///  used with Access to only allows requests from devices with a
///  corresponding client certificate.
///
/// > It's required that an `account_id` or `zone_id` is provided and in
///    most cases using either is fine. However, if you're using a scoped
///    access token, you must provide the argument that matches the token's
///    scope. For example, an access token that is scoped to the "example.com"
///    zone needs to use the `zone_id` argument.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let myCert = access_mutual_tls_certificate::create(
///         "myCert",
///         AccessMutualTlsCertificateArgs::builder()
///             .associated_hostnames(vec!["staging.example.com",])
///             .certificate("${caPem}")
///             .name("My Root Cert")
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Account level import.
///
/// ```sh
/// $ pulumi import cloudflare:index/accessMutualTlsCertificate:AccessMutualTlsCertificate example account/<account_id>/<mutual_tls_certificate_id>
/// ```
///
/// Zone level import.
///
/// ```sh
/// $ pulumi import cloudflare:index/accessMutualTlsCertificate:AccessMutualTlsCertificate example zone/<zone_id>/<mutual_tls_certificate_id>
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod access_mutual_tls_certificate {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccessMutualTlsCertificateArgs {
        /// The account identifier to target for the resource. Conflicts with `zone_id`.
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::Input<Option<String>>,
        /// The hostnames that will be prompted for this certificate.
        #[builder(into, default)]
        pub associated_hostnames: pulumi_gestalt_rust::Input<
            Option<Vec<String>>,
        >,
        /// The Root CA for your certificates.
        #[builder(into, default)]
        pub certificate: pulumi_gestalt_rust::Input<Option<String>>,
        /// The name of the certificate.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::Input<String>,
        /// The zone identifier to target for the resource. Conflicts with `account_id`.
        #[builder(into, default)]
        pub zone_id: pulumi_gestalt_rust::Input<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AccessMutualTlsCertificateResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// The account identifier to target for the resource. Conflicts with `zone_id`.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// The hostnames that will be prompted for this certificate.
        pub associated_hostnames: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The Root CA for your certificates.
        pub certificate: pulumi_gestalt_rust::Output<Option<String>>,
        pub fingerprint: pulumi_gestalt_rust::Output<String>,
        /// The name of the certificate.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The zone identifier to target for the resource. Conflicts with `account_id`.
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AccessMutualTlsCertificateArgs,
    ) -> AccessMutualTlsCertificateResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AccessMutualTlsCertificateArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> AccessMutualTlsCertificateResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AccessMutualTlsCertificateArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> AccessMutualTlsCertificateResult {
        let account_id_binding = args.account_id.get_output(ctx);
        let associated_hostnames_binding = args.associated_hostnames.get_output(ctx);
        let certificate_binding = args.certificate.get_output(ctx);
        let name_binding = args.name.get_output(ctx);
        let zone_id_binding = args.zone_id.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/accessMutualTlsCertificate:AccessMutualTlsCertificate"
                .into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "associatedHostnames".into(),
                    value: &associated_hostnames_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "certificate".into(),
                    value: &certificate_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        AccessMutualTlsCertificateResult {
            id: o.get_id(),
            urn: o.get_urn(),
            account_id: o.get_field("accountId"),
            associated_hostnames: o.get_field("associatedHostnames"),
            certificate: o.get_field("certificate"),
            fingerprint: o.get_field("fingerprint"),
            name: o.get_field("name"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
