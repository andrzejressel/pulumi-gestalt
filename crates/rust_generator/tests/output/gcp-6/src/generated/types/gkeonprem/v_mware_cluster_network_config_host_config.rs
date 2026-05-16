#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VMwareClusterNetworkConfigHostConfig {
    /// DNS search domains.
    /// 
    /// <a name="nested_control_plane_v2_config"></a>The `control_plane_v2_config` block supports:
    #[builder(into)]
    #[serde(rename = "dnsSearchDomains")]
    pub r#dns_search_domains: Option<Vec<String>>,
    /// DNS servers.
    #[builder(into)]
    #[serde(rename = "dnsServers")]
    pub r#dns_servers: Option<Vec<String>>,
    /// NTP servers.
    #[builder(into)]
    #[serde(rename = "ntpServers")]
    pub r#ntp_servers: Option<Vec<String>>,
}
