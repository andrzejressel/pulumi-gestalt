#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NextGenerationFirewallVirtualHubLocalRulestackDnsSettings {
    #[builder(into)]
    #[serde(rename = "azureDnsServers")]
    pub r#azure_dns_servers: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "dnsServers")]
    pub r#dns_servers: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "useAzureDns")]
    pub r#use_azure_dns: Option<bool>,
}
