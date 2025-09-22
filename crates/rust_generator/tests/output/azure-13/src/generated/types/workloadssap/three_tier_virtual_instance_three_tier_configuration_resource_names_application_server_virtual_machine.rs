#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ThreeTierVirtualInstanceThreeTierConfigurationResourceNamesApplicationServerVirtualMachine {
    /// One or more `data_disk` blocks as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "dataDisks")]
    pub r#data_disks: Option<Vec<super::super::types::workloadssap::ThreeTierVirtualInstanceThreeTierConfigurationResourceNamesApplicationServerVirtualMachineDataDisk>>,
    /// The full name of the host of the Virtual Machine. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "hostName")]
    pub r#host_name: Option<String>,
    /// A list of full names for the Network Interface of the Virtual Machine. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "networkInterfaceNames")]
    pub r#network_interface_names: Option<Vec<String>>,
    /// The full name of the OS Disk attached to the Virtual Machine. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "osDiskName")]
    pub r#os_disk_name: Option<String>,
    /// The full name of the Virtual Machine in a single server SAP system. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "virtualMachineName")]
    pub r#virtual_machine_name: Option<String>,
}
