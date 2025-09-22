#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDirectorySelfServicePermission {
    /// Whether WorkSpaces directory users can change the compute type (bundle) for their workspace.
    #[builder(into)]
    #[serde(rename = "changeComputeType")]
    pub r#change_compute_type: bool,
    /// Whether WorkSpaces directory users can increase the volume size of the drives on their workspace.
    #[builder(into)]
    #[serde(rename = "increaseVolumeSize")]
    pub r#increase_volume_size: bool,
    /// Whether WorkSpaces directory users can rebuild the operating system of a workspace to its original state.
    #[builder(into)]
    #[serde(rename = "rebuildWorkspace")]
    pub r#rebuild_workspace: bool,
    /// Whether WorkSpaces directory users can restart their workspace.
    #[builder(into)]
    #[serde(rename = "restartWorkspace")]
    pub r#restart_workspace: bool,
    /// Whether WorkSpaces directory users can switch the running mode of their workspace.
    #[builder(into)]
    #[serde(rename = "switchRunningMode")]
    pub r#switch_running_mode: bool,
}
