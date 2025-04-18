/// Filter expressions that can be referenced across multiple features,
/// e.g. Firewall Rules. See [what is a filter](https://developers.cloudflare.com/firewall/api/cf-filters/what-is-a-filter/)
/// for more details and available fields and operators.
///
/// > `cloudflare.Filter` is in a deprecation phase until January 15th, 2025.
///   During this time period, this resource is still fully
///   supported but you are strongly advised to move to the
///   `cloudflare.Ruleset` resource. Full details can be found in the
///   developer documentation.
///
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let wordpress = filter::create(
///         "wordpress",
///         FilterArgs::builder()
///             .description("Wordpress break-in attempts that are outside of the office")
///             .expression(
///                 "(http.request.uri.path ~ \".*wp-login.php\" or http.request.uri.path ~ \".*xmlrpc.php\") and ip.src ne 192.0.2.1",
///             )
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/filter:Filter example <zone_id>/<filter_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod filter {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct FilterArgs {
        /// A note that you can use to describe the purpose of the filter.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The filter expression to be used.
        #[builder(into)]
        pub expression: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether this filter is currently paused.
        #[builder(into, default)]
        pub paused: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Short reference tag to quickly select related rules.
        #[builder(into, default)]
        pub ref_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct FilterResult {
        /// A note that you can use to describe the purpose of the filter.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The filter expression to be used.
        pub expression: pulumi_gestalt_rust::Output<String>,
        /// Whether this filter is currently paused.
        pub paused: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Short reference tag to quickly select related rules.
        pub ref_: pulumi_gestalt_rust::Output<Option<String>>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: FilterArgs,
    ) -> FilterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let description_binding = args.description.get_output(context);
        let expression_binding = args.expression.get_output(context);
        let paused_binding = args.paused.get_output(context);
        let ref__binding = args.ref_.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/filter:Filter".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: &description_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "expression".into(),
                    value: &expression_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "paused".into(),
                    value: &paused_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ref".into(),
                    value: &ref__binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        FilterResult {
            description: o.get_field("description"),
            expression: o.get_field("expression"),
            paused: o.get_field("paused"),
            ref_: o.get_field("ref"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
