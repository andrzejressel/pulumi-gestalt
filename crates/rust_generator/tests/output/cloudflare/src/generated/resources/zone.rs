/// Provides a Cloudflare Zone resource. Zone is the basic resource for
/// working with Cloudflare and is roughly equivalent to a domain name
/// that the user purchases.
///
/// > If you are attempting to sign up a subdomain of a zone you must first have Subdomain Support entitlement for your account.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = zone::create(
///         "example",
///         ZoneArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .zone("example.com")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/zone:Zone example <zone_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod zone {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct ZoneArgs {
        /// Account ID to manage the zone resource in.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Whether to scan for DNS records on creation. Ignored after zone is created.
        #[builder(into, default)]
        pub jump_start: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Whether this zone is paused (traffic bypasses Cloudflare). Defaults to `false`.
        #[builder(into, default)]
        pub paused: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The name of the commercial plan to apply to the zone. Available values: `free`, `lite`, `pro`, `pro_plus`, `business`, `enterprise`, `partners_free`, `partners_pro`, `partners_business`, `partners_enterprise`.
        #[builder(into, default)]
        pub plan: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A full zone implies that DNS is hosted with Cloudflare. A partial zone is typically a partner-hosted zone or a CNAME setup. Available values: `full`, `partial`, `secondary`. Defaults to `full`.
        #[builder(into, default)]
        pub type_: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of Vanity Nameservers (if set).
        #[builder(into, default)]
        pub vanity_name_servers: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The DNS zone name which will be added. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct ZoneResult {
        /// Account ID to manage the zone resource in.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// Whether to scan for DNS records on creation. Ignored after zone is created.
        pub jump_start: pulumi_gestalt_rust::Output<Option<bool>>,
        pub meta: pulumi_gestalt_rust::Output<std::collections::HashMap<String, bool>>,
        /// Cloudflare-assigned name servers. This is only populated for zones that use Cloudflare DNS.
        pub name_servers: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Whether this zone is paused (traffic bypasses Cloudflare). Defaults to `false`.
        pub paused: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The name of the commercial plan to apply to the zone. Available values: `free`, `lite`, `pro`, `pro_plus`, `business`, `enterprise`, `partners_free`, `partners_pro`, `partners_business`, `partners_enterprise`.
        pub plan: pulumi_gestalt_rust::Output<String>,
        /// Status of the zone. Available values: `active`, `pending`, `initializing`, `moved`, `deleted`, `deactivated`.
        pub status: pulumi_gestalt_rust::Output<String>,
        /// A full zone implies that DNS is hosted with Cloudflare. A partial zone is typically a partner-hosted zone or a CNAME setup. Available values: `full`, `partial`, `secondary`. Defaults to `full`.
        pub type_: pulumi_gestalt_rust::Output<Option<String>>,
        /// List of Vanity Nameservers (if set).
        pub vanity_name_servers: pulumi_gestalt_rust::Output<Vec<String>>,
        /// Contains the TXT record value to validate domain ownership. This is only populated for zones of type `partial`.
        pub verification_key: pulumi_gestalt_rust::Output<String>,
        /// The DNS zone name which will be added. **Modifying this attribute will force creation of a new resource.**
        pub zone: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: ZoneArgs,
    ) -> ZoneResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let jump_start_binding = args.jump_start.get_output(context);
        let paused_binding = args.paused.get_output(context);
        let plan_binding = args.plan.get_output(context);
        let type__binding = args.type_.get_output(context);
        let vanity_name_servers_binding = args.vanity_name_servers.get_output(context);
        let zone_binding = args.zone.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/zone:Zone".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "jumpStart".into(),
                    value: &jump_start_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "paused".into(),
                    value: &paused_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "plan".into(),
                    value: &plan_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "type".into(),
                    value: &type__binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vanityNameServers".into(),
                    value: &vanity_name_servers_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zone".into(),
                    value: &zone_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        ZoneResult {
            account_id: o.get_field("accountId"),
            jump_start: o.get_field("jumpStart"),
            meta: o.get_field("meta"),
            name_servers: o.get_field("nameServers"),
            paused: o.get_field("paused"),
            plan: o.get_field("plan"),
            status: o.get_field("status"),
            type_: o.get_field("type"),
            vanity_name_servers: o.get_field("vanityNameServers"),
            verification_key: o.get_field("verificationKey"),
            zone: o.get_field("zone"),
        }
    }
}
