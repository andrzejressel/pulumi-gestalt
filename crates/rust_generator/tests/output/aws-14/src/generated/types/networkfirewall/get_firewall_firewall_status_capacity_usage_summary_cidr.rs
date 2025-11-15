#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetFirewallFirewallStatusCapacityUsageSummaryCidr {
    /// Available number of CIDR blocks available for use by the IP set references in a firewall.
    #[builder(into)]
    #[serde(rename = "availableCidrCount")]
    pub r#available_cidr_count: i32,
    /// The list of IP set references used by a firewall.
    #[builder(into)]
    #[serde(rename = "ipSetReferences")]
    pub r#ip_set_references: Vec<super::super::types::networkfirewall::GetFirewallFirewallStatusCapacityUsageSummaryCidrIpSetReference>,
    /// Number of CIDR blocks used by the IP set references in a firewall.
    #[builder(into)]
    #[serde(rename = "utilizedCidrCount")]
    pub r#utilized_cidr_count: i32,
}
