#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AccessLevelBasicConditionVpcNetworkSourceVpcSubnetwork {
    /// Required. Network name to be allowed by this Access Level. Networks of foreign organizations requires `compute.network.get` permission to be granted to caller.
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: String,
    /// CIDR block IP subnetwork specification. Must be IPv4.
    #[builder(into)]
    #[serde(rename = "vpcIpSubnetworks")]
    pub r#vpc_ip_subnetworks: Option<Vec<String>>,
}
