#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetNetworkSimStaticIpConfiguration {
    /// The ID of attached data network on which the static.
    #[builder(into)]
    #[serde(rename = "attachedDataNetworkId")]
    pub r#attached_data_network_id: String,
    #[builder(into)]
    #[serde(rename = "sliceId")]
    pub r#slice_id: String,
    /// The IPv4 address assigned to the SIM at this network scope.
    #[builder(into)]
    #[serde(rename = "staticIpv4Address")]
    pub r#static_ipv_4_address: String,
}
