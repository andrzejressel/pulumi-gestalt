/// Provides a Cloudflare Zone Hold resource that prevents adding
/// the hostname to another account for use.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = zone_hold::create(
///         "example",
///         ZoneHoldArgs::builder()
///             .hold(true)
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/zoneHold:ZoneHold example <zone_id>
/// ```
///
#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod zone_hold {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZoneHoldArgs {
        /// Enablement status of the zone hold.
        #[builder(into)]
        pub hold: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// The RFC3339 compatible timestamp when to automatically re-enable the zone hold.
        #[builder(into, default)]
        pub hold_after: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Whether to extend to block any subdomain of the given zone.
        #[builder(into, default)]
        pub include_subdomains: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The zone identifier to target for the resource.
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ZoneHoldResult {
        /// Pulumi ID is the provider-assigned unique ID for this managed resource.
        /// It is set during deployments and may be missing (unknown) during planning phases.
        pub id: pulumi_gestalt_rust::Output<String>,
        /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
        pub urn: pulumi_gestalt_rust::Output<String>,
        /// Enablement status of the zone hold.
        pub hold: pulumi_gestalt_rust::Output<bool>,
        /// The RFC3339 compatible timestamp when to automatically re-enable the zone hold.
        pub hold_after: pulumi_gestalt_rust::Output<String>,
        /// Whether to extend to block any subdomain of the given zone.
        pub include_subdomains: pulumi_gestalt_rust::Output<Option<bool>>,
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
        args: ZoneHoldArgs,
    ) -> ZoneHoldResult {
        __create(ctx, name, args, None)
    }
    ///
    /// Same as `create`, but with additional generic options that control the behavior of the resource registration.
    ///
    #[allow(non_snake_case, dead_code)]
    pub fn create_with_options(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ZoneHoldArgs,
        options: pulumi_gestalt_rust::CustomResourceOptions,
    ) -> ZoneHoldResult {
        __create(ctx, name, args, Some(options))
    }
    #[allow(non_snake_case, unused_imports, dead_code)]
    fn __create(
        ctx: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ZoneHoldArgs,
        options: Option<pulumi_gestalt_rust::CustomResourceOptions>,
    ) -> ZoneHoldResult {
        let hold_binding = args.hold.get_output(ctx);
        let hold_after_binding = args.hold_after.get_output(ctx);
        let include_subdomains_binding = args.include_subdomains.get_output(ctx);
        let zone_id_binding = args.zone_id.get_output(ctx);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/zoneHold:ZoneHold".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "hold".into(),
                    value: &hold_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "holdAfter".into(),
                    value: &hold_after_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "includeSubdomains".into(),
                    value: &include_subdomains_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding.drop_type(),
                },
            ],
            options,
        };
        let o = ctx.register_resource(request);
        ZoneHoldResult {
            id: o.get_id(),
            urn: o.get_urn(),
            hold: o.get_field("hold"),
            hold_after: o.get_field("holdAfter"),
            include_subdomains: o.get_field("includeSubdomains"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
