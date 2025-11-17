#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualMachineOsProfileWindowsConfig {
    /// An `additional_unattend_config` block as defined below.
    #[builder(into)]
    #[serde(rename = "additionalUnattendConfigs")]
    pub r#additional_unattend_configs: Option<Vec<super::super::types::compute::VirtualMachineOsProfileWindowsConfigAdditionalUnattendConfig>>,
    /// Are automatic updates enabled on this Virtual Machine? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "enableAutomaticUpgrades")]
    pub r#enable_automatic_upgrades: Option<bool>,
    /// Should the Azure Virtual Machine Guest Agent be installed on this Virtual Machine? Defaults to `false`.
    /// 
    /// > **NOTE:** This is different from the Default value used for this field within Azure.
    #[builder(into)]
    #[serde(rename = "provisionVmAgent")]
    pub r#provision_vm_agent: Option<bool>,
    /// Specifies the time zone of the virtual machine, [the possible values are defined here](https://jackstromberg.com/2017/01/list-of-time-zones-consumed-by-azure/). Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "timezone")]
    pub r#timezone: Option<String>,
    /// One or more `winrm` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "winrms")]
    pub r#winrms: Option<Vec<super::super::types::compute::VirtualMachineOsProfileWindowsConfigWinrm>>,
}
