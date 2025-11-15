#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceUpgradeHistory {
    /// Optional. Action. Rolloback or Upgrade.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Option<String>,
    /// Optional. The container image before this instance upgrade.
    #[builder(into)]
    #[serde(rename = "containerImage")]
    pub r#container_image: Option<String>,
    /// An RFC3339 timestamp in UTC time. This in the format of yyyy-MM-ddTHH:mm:ss.SSSZ.
    /// The milliseconds portion (".SSS") is optional.
    #[builder(into)]
    #[serde(rename = "createTime")]
    pub r#create_time: Option<String>,
    /// Optional. The framework of this workbench instance.
    #[builder(into)]
    #[serde(rename = "framework")]
    pub r#framework: Option<String>,
    /// Optional. The snapshot of the boot disk of this workbench instance before upgrade.
    #[builder(into)]
    #[serde(rename = "snapshot")]
    pub r#snapshot: Option<String>,
    /// (Output)
    /// Output only. The state of this instance upgrade history entry.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Option<String>,
    /// Optional. Target VM Version, like m63.
    #[builder(into)]
    #[serde(rename = "targetVersion")]
    pub r#target_version: Option<String>,
    /// Optional. The version of the workbench instance before this upgrade.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Option<String>,
    /// Optional. The VM image before this instance upgrade.
    #[builder(into)]
    #[serde(rename = "vmImage")]
    pub r#vm_image: Option<String>,
}
