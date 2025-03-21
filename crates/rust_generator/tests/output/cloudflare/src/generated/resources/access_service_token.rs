/// Access Service Tokens are used for service-to-service communication
/// when an application is behind Cloudflare Access.
///
/// ## Import
///
/// If you are importing an Access Service Token you will not have the
///
/// client_secret available in the state for use. The client_secret is only
///
/// available once, at creation. In most cases, it is better to just create a new
///
/// resource should you need to reference it in other resources.
///
/// ```sh
/// $ pulumi import cloudflare:index/accessServiceToken:AccessServiceToken example <account_id>/<service_token_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod access_service_token {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct AccessServiceTokenArgs {
        /// The account identifier to target for the resource. Conflicts with `zone_id`.
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Length of time the service token is valid for. Available values: `8760h`, `17520h`, `43800h`, `87600h`, `forever`.
        #[builder(into, default)]
        pub duration: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        #[builder(into, default)]
        pub min_days_for_renewal: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Friendly name of the token's intent.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The zone identifier to target for the resource. Conflicts with `account_id`.
        #[builder(into, default)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct AccessServiceTokenResult {
        /// The account identifier to target for the resource. Conflicts with `zone_id`.
        pub account_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Client ID associated with the Service Token. **Modifying this attribute will force creation of a new resource.**
        pub client_id: pulumi_gestalt_rust::Output<String>,
        /// A secret for interacting with Access protocols. **Modifying this attribute will force creation of a new resource.**
        pub client_secret: pulumi_gestalt_rust::Output<String>,
        /// Length of time the service token is valid for. Available values: `8760h`, `17520h`, `43800h`, `87600h`, `forever`.
        pub duration: pulumi_gestalt_rust::Output<String>,
        /// Date when the token expires.
        pub expires_at: pulumi_gestalt_rust::Output<String>,
        pub min_days_for_renewal: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Friendly name of the token's intent.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The zone identifier to target for the resource. Conflicts with `account_id`.
        pub zone_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: AccessServiceTokenArgs,
    ) -> AccessServiceTokenResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let duration_binding = args.duration.get_output(context);
        let min_days_for_renewal_binding = args.min_days_for_renewal.get_output(context);
        let name_binding = args.name.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/accessServiceToken:AccessServiceToken".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "duration".into(),
                    value: &duration_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "minDaysForRenewal".into(),
                    value: &min_days_for_renewal_binding.drop_type(),
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
        };
        let o = context.register_resource(request);
        AccessServiceTokenResult {
            account_id: o.get_field("accountId"),
            client_id: o.get_field("clientId"),
            client_secret: o.get_field("clientSecret"),
            duration: o.get_field("duration"),
            expires_at: o.get_field("expiresAt"),
            min_days_for_renewal: o.get_field("minDaysForRenewal"),
            name: o.get_field("name"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
