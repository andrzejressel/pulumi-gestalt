#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SlotSiteConfigScmIpRestriction {
    /// Allow or Deny access for this IP range. Defaults to `Allow`.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Option<String>,
    /// The `headers` block for this specific `scm_ip_restriction` as defined below.
    #[builder(into)]
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<super::super::types::appservice::SlotSiteConfigScmIpRestrictionHeaders>>,
    /// The IP Address used for this IP Restriction in CIDR notation.
    #[builder(into)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: Option<String>,
    /// The name for this IP Restriction.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The priority for this IP Restriction. Restrictions are enforced in priority order. By default, priority is set to 65000 if not specified.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Option<i32>,
    /// The Service Tag used for this IP Restriction.
    #[builder(into)]
    #[serde(rename = "serviceTag")]
    pub r#service_tag: Option<String>,
    /// The Virtual Network Subnet ID used for this IP Restriction.
    /// 
    /// > **NOTE:** One of either `ip_address`, `service_tag` or `virtual_network_subnet_id` must be specified
    #[builder(into)]
    #[serde(rename = "virtualNetworkSubnetId")]
    pub r#virtual_network_subnet_id: Option<String>,
}
