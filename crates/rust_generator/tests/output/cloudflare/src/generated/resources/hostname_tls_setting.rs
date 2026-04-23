/// Provides a Cloudflare per-hostname TLS setting resource. Used to set TLS settings for hostnames under the specified zone.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = hostname_tls_setting::create(
///         "example",
///         HostnameTlsSettingArgs::builder()
///             .hostname("sub.example.com")
///             .setting("min_tls_version")
///             .value("1.2")
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/hostnameTlsSetting:HostnameTlsSetting example <zone_id>/<hostname>/<setting_name>
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod hostname_tls_setting {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct HostnameTlsSettingArgs {
        /// Hostname that belongs to this zone name. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub hostname: pulumi_gestalt_rust::Input<String>,
        /// TLS setting name. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub setting: pulumi_gestalt_rust::Input<String>,
        /// TLS setting value.
        #[builder(into)]
        pub value: pulumi_gestalt_rust::Input<String>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::Input<String>,
    }
    #[allow(dead_code)]
    pub struct HostnameTlsSettingResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        pub created_at: pulumi_gestalt_rust::Output<String>,
        /// Hostname that belongs to this zone name. **Modifying this attribute will force creation of a new resource.**
        pub hostname: pulumi_gestalt_rust::Output<String>,
        /// TLS setting name. **Modifying this attribute will force creation of a new resource.**
        pub setting: pulumi_gestalt_rust::Output<String>,
        pub updated_at: pulumi_gestalt_rust::Output<String>,
        /// TLS setting value.
        pub value: pulumi_gestalt_rust::Output<String>,
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
        args: HostnameTlsSettingArgs,
    ) -> HostnameTlsSettingResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: HostnameTlsSettingArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> HostnameTlsSettingResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: HostnameTlsSettingArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> HostnameTlsSettingResult {
        let hostname_binding = args.hostname.get_output(ctx);
        let setting_binding = args.setting.get_output(ctx);
        let value_binding = args.value.get_output(ctx);
        let zone_id_binding = args.zone_id.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/hostnameTlsSetting:HostnameTlsSetting".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hostname".into(),
                    value: &hostname_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "setting".into(),
                    value: &setting_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "value".into(),
                    value: &value_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        HostnameTlsSettingResult {
            id: o.get_id(),
            urn: o.get_urn(),
            created_at: o.get_field("createdAt"),
            hostname: o.get_field("hostname"),
            setting: o.get_field("setting"),
            updated_at: o.get_field("updatedAt"),
            value: o.get_field("value"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
