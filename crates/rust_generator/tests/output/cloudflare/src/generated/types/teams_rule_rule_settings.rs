#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TeamsRuleRuleSettings {
    /// Add custom headers to allowed requests in the form of key-value pairs.
    #[builder(into)]
    #[serde(rename = "addHeaders")]
    pub r#add_headers: Option<std::collections::HashMap<String, String>>,
    /// Allow parent MSP accounts to enable bypass their children's rules.
    #[builder(into)]
    #[serde(rename = "allowChildBypass")]
    pub r#allow_child_bypass: Option<bool>,
    /// Settings for auditing SSH usage.
    #[builder(into)]
    #[serde(rename = "auditSsh")]
    pub r#audit_ssh: Option<Box<super::types::TeamsRuleRuleSettingsAuditSsh>>,
    /// Configure how browser isolation behaves.
    #[builder(into)]
    #[serde(rename = "bisoAdminControls")]
    pub r#biso_admin_controls: Option<Box<super::types::TeamsRuleRuleSettingsBisoAdminControls>>,
    /// Indicator of block page enablement.
    #[builder(into)]
    #[serde(rename = "blockPageEnabled")]
    pub r#block_page_enabled: Option<bool>,
    /// The displayed reason for a user being blocked.
    #[builder(into)]
    #[serde(rename = "blockPageReason")]
    pub r#block_page_reason: Option<String>,
    /// Allow child MSP accounts to bypass their parent's rule.
    #[builder(into)]
    #[serde(rename = "bypassParentRule")]
    pub r#bypass_parent_rule: Option<bool>,
    /// Configure how session check behaves.
    #[builder(into)]
    #[serde(rename = "checkSession")]
    pub r#check_session: Option<Box<super::types::TeamsRuleRuleSettingsCheckSession>>,
    /// Add your own custom resolvers to route queries that match the resolver policy. Cannot be used when resolve*dns*through*cloudflare is set. DNS queries will route to the address closest to their origin.
    #[builder(into)]
    #[serde(rename = "dnsResolvers")]
    pub r#dns_resolvers: Option<Box<super::types::TeamsRuleRuleSettingsDnsResolvers>>,
    /// Configure how Proxy traffic egresses. Can be set for rules with Egress action and Egress filter. Can be omitted to indicate local egress via Warp IPs.
    #[builder(into)]
    #[serde(rename = "egress")]
    pub r#egress: Option<Box<super::types::TeamsRuleRuleSettingsEgress>>,
    /// Set to true, to ignore the category matches at CNAME domains in a response.
    #[builder(into)]
    #[serde(rename = "ignoreCnameCategoryMatches")]
    pub r#ignore_cname_category_matches: Option<bool>,
    /// Disable DNSSEC validation (must be Allow rule).
    #[builder(into)]
    #[serde(rename = "insecureDisableDnssecValidation")]
    pub r#insecure_disable_dnssec_validation: Option<bool>,
    /// Turns on IP category based filter on dns if the rule contains dns category checks.
    #[builder(into)]
    #[serde(rename = "ipCategories")]
    pub r#ip_categories: Option<bool>,
    /// Settings to forward layer 4 traffic.
    #[builder(into)]
    #[serde(rename = "l4override")]
    pub r#l_4_override: Option<Box<super::types::TeamsRuleRuleSettingsL4Override>>,
    /// Notification settings on a block rule.
    #[builder(into)]
    #[serde(rename = "notificationSettings")]
    pub r#notification_settings: Option<Box<super::types::TeamsRuleRuleSettingsNotificationSettings>>,
    /// The host to override matching DNS queries with.
    #[builder(into)]
    #[serde(rename = "overrideHost")]
    pub r#override_host: Option<String>,
    /// The IPs to override matching DNS queries with.
    #[builder(into)]
    #[serde(rename = "overrideIps")]
    pub r#override_ips: Option<Vec<String>>,
    /// Configure DLP Payload Logging settings for this rule.
    #[builder(into)]
    #[serde(rename = "payloadLog")]
    pub r#payload_log: Option<Box<super::types::TeamsRuleRuleSettingsPayloadLog>>,
    /// Enable sending queries that match the resolver policy to Cloudflare's default 1.1.1.1 DNS resolver. Cannot be set when `dns_resolvers` are specified.
    #[builder(into)]
    #[serde(rename = "resolveDnsThroughCloudflare")]
    pub r#resolve_dns_through_cloudflare: Option<bool>,
    /// Configure untrusted certificate settings for this rule.
    #[builder(into)]
    #[serde(rename = "untrustedCert")]
    pub r#untrusted_cert: Option<Box<super::types::TeamsRuleRuleSettingsUntrustedCert>>,
}
