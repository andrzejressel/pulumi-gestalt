#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PatchDeploymentPatchConfig {
    /// Apt update settings. Use this setting to override the default apt patch rules.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "apt")]
    pub r#apt: Option<Box<super::super::types::osconfig::PatchDeploymentPatchConfigApt>>,
    /// goo update settings. Use this setting to override the default goo patch rules.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "goo")]
    pub r#goo: Option<Box<super::super::types::osconfig::PatchDeploymentPatchConfigGoo>>,
    /// Allows the patch job to run on Managed instance groups (MIGs).
    #[builder(into)]
    #[serde(rename = "migInstancesAllowed")]
    pub r#mig_instances_allowed: Option<bool>,
    /// The ExecStep to run after the patch update.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "postStep")]
    pub r#post_step: Option<Box<super::super::types::osconfig::PatchDeploymentPatchConfigPostStep>>,
    /// The ExecStep to run before the patch update.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "preStep")]
    pub r#pre_step: Option<Box<super::super::types::osconfig::PatchDeploymentPatchConfigPreStep>>,
    /// Post-patch reboot settings.
    /// Possible values are: `DEFAULT`, `ALWAYS`, `NEVER`.
    #[builder(into)]
    #[serde(rename = "rebootConfig")]
    pub r#reboot_config: Option<String>,
    /// Windows update settings. Use this setting to override the default Windows patch rules.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "windowsUpdate")]
    pub r#windows_update: Option<Box<super::super::types::osconfig::PatchDeploymentPatchConfigWindowsUpdate>>,
    /// Yum update settings. Use this setting to override the default yum patch rules.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "yum")]
    pub r#yum: Option<Box<super::super::types::osconfig::PatchDeploymentPatchConfigYum>>,
    /// zypper update settings. Use this setting to override the default zypper patch rules.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "zypper")]
    pub r#zypper: Option<Box<super::super::types::osconfig::PatchDeploymentPatchConfigZypper>>,
}
