#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HciDeploymentSettingScaleUnitHostNetworkStorageNetwork {
    /// The name of the storage network. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The name of the network adapter. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "networkAdapterName")]
    pub r#network_adapter_name: String,
    /// Specifies the ID for the VLAN storage network. This setting is applied to the network interfaces that route the storage and VM migration traffic. Changing this forces a new Stack HCI Deployment Setting to be created.
    #[builder(into)]
    #[serde(rename = "vlanId")]
    pub r#vlan_id: String,
}
