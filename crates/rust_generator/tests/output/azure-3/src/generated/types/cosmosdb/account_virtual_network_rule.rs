#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AccountVirtualNetworkRule {
    /// The ID of the virtual network subnet.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// If set to true, the specified subnet will be added as a virtual network rule even if its CosmosDB service endpoint is not active. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "ignoreMissingVnetServiceEndpoint")]
    pub r#ignore_missing_vnet_service_endpoint: Option<bool>,
}
