#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NetworkSimPolicySlice {
    /// An array of `data_network` block as defined below.
    #[builder(into)]
    #[serde(rename = "dataNetworks")]
    pub r#data_networks: Vec<super::super::types::mobile::NetworkSimPolicySliceDataNetwork>,
    /// The ID of default data network to use if the user equipment does not explicitly specify it. Configuration for this object must exist in the `data_network` block.
    #[builder(into)]
    #[serde(rename = "defaultDataNetworkId")]
    pub r#default_data_network_id: String,
    /// The ID of the slice that these settings apply to.
    #[builder(into)]
    #[serde(rename = "sliceId")]
    pub r#slice_id: String,
}
