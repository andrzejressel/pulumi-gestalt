#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceGceSetupNetworkInterface {
    /// Optional. An array of configurations for this interface. Currently, only one access
    /// config, ONE_TO_ONE_NAT, is supported. If no accessConfigs specified, the
    /// instance will have an external internet access through an ephemeral
    /// external IP address.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "accessConfigs")]
    pub r#access_configs: Option<Vec<super::super::types::workbench::InstanceGceSetupNetworkInterfaceAccessConfig>>,
    /// Optional. The name of the VPC that this VM instance is in.
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: Option<String>,
    /// Optional. The type of vNIC to be used on this interface. This
    /// may be gVNIC or VirtioNet.
    /// Possible values are: `VIRTIO_NET`, `GVNIC`.
    #[builder(into)]
    #[serde(rename = "nicType")]
    pub r#nic_type: Option<String>,
    /// Optional. The name of the subnet that this VM instance is in.
    #[builder(into)]
    #[serde(rename = "subnet")]
    pub r#subnet: Option<String>,
}
