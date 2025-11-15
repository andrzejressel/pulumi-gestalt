#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NextGenerationFirewallVirtualNetworkPanoramaPanorama {
    /// The Device Group Name to which this Firewall Resource is registered.
    #[builder(into)]
    #[serde(rename = "deviceGroupName")]
    pub r#device_group_name: Option<String>,
    /// The Host Name of this Firewall Resource.
    #[builder(into)]
    #[serde(rename = "hostName")]
    pub r#host_name: Option<String>,
    /// The name which should be used for this Palo Alto Next Generation Firewall Virtual Network Panorama. Changing this forces a new Palo Alto Next Generation Firewall Virtual Network Panorama to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The name of the First Panorana server.
    #[builder(into)]
    #[serde(rename = "panoramaServer1")]
    pub r#panorama_server_1: Option<String>,
    /// The name of the Second Panorana server.
    #[builder(into)]
    #[serde(rename = "panoramaServer2")]
    pub r#panorama_server_2: Option<String>,
    /// The name of the Panorama Template applied to this Firewall Resource.
    #[builder(into)]
    #[serde(rename = "templateName")]
    pub r#template_name: Option<String>,
    /// The SSH Key to connect to the Firewall Resource.
    #[builder(into)]
    #[serde(rename = "virtualMachineSshKey")]
    pub r#virtual_machine_ssh_key: Option<String>,
}
