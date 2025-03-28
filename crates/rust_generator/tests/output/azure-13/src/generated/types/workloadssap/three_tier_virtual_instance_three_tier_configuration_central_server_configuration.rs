#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ThreeTierVirtualInstanceThreeTierConfigurationCentralServerConfiguration {
    /// The number of instances for the Central Server. Possible values are at least `1`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "instanceCount")]
    pub r#instance_count: Box<i32>,
    /// The resource ID of the Subnet for the Central Server. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<String>,
    /// A `virtual_machine_configuration` block as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "virtualMachineConfiguration")]
    pub r#virtual_machine_configuration: Box<super::super::types::workloadssap::ThreeTierVirtualInstanceThreeTierConfigurationCentralServerConfigurationVirtualMachineConfiguration>,
}
