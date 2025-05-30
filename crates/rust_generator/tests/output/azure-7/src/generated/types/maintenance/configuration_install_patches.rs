#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConfigurationInstallPatches {
    /// A `linux` block as defined above. This property only applies when `scope` is set to `InGuestPatch`
    #[builder(into, default)]
    #[serde(rename = "linuxes")]
    pub r#linuxes: Box<Option<Vec<super::super::types::maintenance::ConfigurationInstallPatchesLinux>>>,
    /// Possible reboot preference as defined by the user based on which it would be decided to reboot the machine or not after the patch operation is completed. Possible values are `Always`, `IfRequired` and `Never`. This property only applies when `scope` is set to `InGuestPatch`.
    #[builder(into, default)]
    #[serde(rename = "reboot")]
    pub r#reboot: Box<Option<String>>,
    /// A `windows` block as defined above. This property only applies when `scope` is set to `InGuestPatch`
    #[builder(into, default)]
    #[serde(rename = "windows")]
    pub r#windows: Box<Option<Vec<super::super::types::maintenance::ConfigurationInstallPatchesWindow>>>,
}
