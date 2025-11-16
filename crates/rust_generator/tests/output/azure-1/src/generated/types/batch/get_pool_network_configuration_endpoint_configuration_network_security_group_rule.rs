#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetPoolNetworkConfigurationEndpointConfigurationNetworkSecurityGroupRule {
    /// The action that should be taken for a specified IP address, subnet range or tag.
    #[builder(into)]
    #[serde(rename = "access")]
    pub r#access: String,
    /// The priority for this rule.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: i32,
    /// The source address prefix or tag to match for the rule.
    #[builder(into)]
    #[serde(rename = "sourceAddressPrefix")]
    pub r#source_address_prefix: String,
    /// The source port ranges to match for the rule.
    #[builder(into)]
    #[serde(rename = "sourcePortRanges")]
    pub r#source_port_ranges: Vec<String>,
}
