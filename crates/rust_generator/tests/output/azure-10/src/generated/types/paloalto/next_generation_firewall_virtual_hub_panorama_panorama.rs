#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NextGenerationFirewallVirtualHubPanoramaPanorama {
    #[builder(into)]
    #[serde(rename = "deviceGroupName")]
    pub r#device_group_name: Option<String>,
    #[builder(into)]
    #[serde(rename = "hostName")]
    pub r#host_name: Option<String>,
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    #[builder(into)]
    #[serde(rename = "panoramaServer1")]
    pub r#panorama_server_1: Option<String>,
    #[builder(into)]
    #[serde(rename = "panoramaServer2")]
    pub r#panorama_server_2: Option<String>,
    #[builder(into)]
    #[serde(rename = "templateName")]
    pub r#template_name: Option<String>,
    #[builder(into)]
    #[serde(rename = "virtualMachineSshKey")]
    pub r#virtual_machine_ssh_key: Option<String>,
}
