#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SingleNodeVirtualInstanceSingleServerConfiguration {
    #[builder(into)]
    #[serde(rename = "appResourceGroupName")]
    pub r#app_resource_group_name: String,
    /// The supported SAP database type. Possible values are `DB2` and `HANA`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "databaseType")]
    pub r#database_type: Option<String>,
    /// One or more `disk_volume_configuration` blocks as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "diskVolumeConfigurations")]
    pub r#disk_volume_configurations: Option<Vec<super::super::types::workloadssap::SingleNodeVirtualInstanceSingleServerConfigurationDiskVolumeConfiguration>>,
    /// Specifies whether a secondary IP address should be added to the network interface on all VMs of the SAP system being deployed. Defaults to `false`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "secondaryIpEnabled")]
    pub r#secondary_ip_enabled: Option<bool>,
    /// The resource ID of the Subnet for the SAP Single Node Virtual Instance. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: String,
    /// A `virtual_machine_configuration` block as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "virtualMachineConfiguration")]
    pub r#virtual_machine_configuration: Box<super::super::types::workloadssap::SingleNodeVirtualInstanceSingleServerConfigurationVirtualMachineConfiguration>,
    /// A `virtual_machine_resource_names` block as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "virtualMachineResourceNames")]
    pub r#virtual_machine_resource_names: Option<Box<super::super::types::workloadssap::SingleNodeVirtualInstanceSingleServerConfigurationVirtualMachineResourceNames>>,
}
