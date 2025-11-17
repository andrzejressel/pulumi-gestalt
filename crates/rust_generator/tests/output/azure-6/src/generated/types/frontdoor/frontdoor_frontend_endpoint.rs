#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FrontdoorFrontendEndpoint {
    /// Specifies the host name of the `frontend_endpoint`. Must be a domain name. In order to use a name.azurefd.net domain, the name value must match the Front Door name.
    #[builder(into)]
    #[serde(rename = "hostName")]
    pub r#host_name: String,
    /// The ID of the FrontDoor.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// Specifies the name of the `frontend_endpoint`.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Whether to allow session affinity on this host. Valid options are `true` or `false` Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "sessionAffinityEnabled")]
    pub r#session_affinity_enabled: Option<bool>,
    /// The TTL to use in seconds for session affinity, if applicable. Defaults to `0`.
    #[builder(into)]
    #[serde(rename = "sessionAffinityTtlSeconds")]
    pub r#session_affinity_ttl_seconds: Option<i32>,
    /// Defines the Web Application Firewall policy `ID` for each host.
    #[builder(into)]
    #[serde(rename = "webApplicationFirewallPolicyLinkId")]
    pub r#web_application_firewall_policy_link_id: Option<String>,
}
