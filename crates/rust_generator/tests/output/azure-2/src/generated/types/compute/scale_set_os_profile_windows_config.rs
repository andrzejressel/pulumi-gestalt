#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ScaleSetOsProfileWindowsConfig {
    /// An `additional_unattend_config` block as documented below.
    #[builder(into, default)]
    #[serde(rename = "additionalUnattendConfigs")]
    pub r#additional_unattend_configs: Box<Option<Vec<super::super::types::compute::ScaleSetOsProfileWindowsConfigAdditionalUnattendConfig>>>,
    /// Indicates whether virtual machines in the scale set are enabled for automatic updates.
    #[builder(into, default)]
    #[serde(rename = "enableAutomaticUpgrades")]
    pub r#enable_automatic_upgrades: Box<Option<bool>>,
    /// Indicates whether virtual machine agent should be provisioned on the virtual machines in the scale set.
    #[builder(into, default)]
    #[serde(rename = "provisionVmAgent")]
    pub r#provision_vm_agent: Box<Option<bool>>,
    /// A collection of `winrm` blocks as documented below.
    #[builder(into, default)]
    #[serde(rename = "winrms")]
    pub r#winrms: Box<Option<Vec<super::super::types::compute::ScaleSetOsProfileWindowsConfigWinrm>>>,
}
