#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NetworkPacketCoreControlPlanePlatform {
    /// The ID of the Azure Arc connected cluster where the packet core is deployed.
    #[builder(into)]
    #[serde(rename = "arcKubernetesClusterId")]
    pub r#arc_kubernetes_cluster_id: Option<String>,
    /// The ID of the Azure Arc custom location where the packet core is deployed.
    /// 
    /// > **NOTE:** At least one of `edge_device_id`, `arc_kubernetes_cluster_id`, `stack_hci_cluster_id` and `custom_location_id` should be specified. If multiple are set, they must be consistent with each other.
    #[builder(into)]
    #[serde(rename = "customLocationId")]
    pub r#custom_location_id: Option<String>,
    /// The ID of the Azure Stack Edge device where the packet core is deployed. If the device is part of a fault-tolerant pair, either device in the pair can be specified.
    #[builder(into)]
    #[serde(rename = "edgeDeviceId")]
    pub r#edge_device_id: Option<String>,
    /// The ID of the Azure Stack HCI cluster where the packet core is deployed.
    #[builder(into)]
    #[serde(rename = "stackHciClusterId")]
    pub r#stack_hci_cluster_id: Option<String>,
    /// Specifies the platform type where the packet core is deployed. Possible values are `AKS-HCI`, `3P-AZURE-STACK-HCI` and `BaseVM`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
