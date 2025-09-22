#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetWindowsFunctionAppSiteConfigIpRestriction {
    /// The action to take.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: String,
    /// The description of the ip restriction rule.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: String,
    #[builder(into)]
    #[serde(rename = "headers")]
    pub r#headers: Vec<super::super::types::appservice::GetWindowsFunctionAppSiteConfigIpRestrictionHeader>,
    /// The CIDR notation of the IP or IP Range to match.
    #[builder(into)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: String,
    /// The name of this Windows Function App.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The priority value of this `ip_restriction`.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: i32,
    /// The Service Tag used for this IP Restriction.
    #[builder(into)]
    #[serde(rename = "serviceTag")]
    pub r#service_tag: String,
    /// The subnet id which the Windows Function App is vNet Integrated with.
    #[builder(into)]
    #[serde(rename = "virtualNetworkSubnetId")]
    pub r#virtual_network_subnet_id: String,
}
