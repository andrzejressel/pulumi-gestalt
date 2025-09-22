#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SpaceSpaceSettingsJupyterLabAppSettingsAppLifecycleManagement {
    /// Settings related to idle shutdown of Studio applications. See `idle_settings` Block below.
    #[builder(into)]
    #[serde(rename = "idleSettings")]
    pub r#idle_settings: Box<Option<super::super::types::sagemaker::SpaceSpaceSettingsJupyterLabAppSettingsAppLifecycleManagementIdleSettings>>,
}
