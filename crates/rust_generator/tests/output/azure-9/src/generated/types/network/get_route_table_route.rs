#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetRouteTableRoute {
    /// The destination CIDR to which the route applies.
    #[builder(into)]
    #[serde(rename = "addressPrefix")]
    pub r#address_prefix: String,
    /// The name of the Route Table.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Contains the IP address packets should be forwarded to.
    #[builder(into)]
    #[serde(rename = "nextHopInIpAddress")]
    pub r#next_hop_in_ip_address: String,
    /// The type of Azure hop the packet should be sent to.
    #[builder(into)]
    #[serde(rename = "nextHopType")]
    pub r#next_hop_type: String,
}
