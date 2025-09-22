#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WindowsWebAppSiteConfigAutoHealSettingActionCustomAction {
    /// The executable to run for the `custom_action`.
    #[builder(into)]
    #[serde(rename = "executable")]
    pub r#executable: String,
    /// The parameters to pass to the specified `executable`.
    #[builder(into)]
    #[serde(rename = "parameters")]
    pub r#parameters: Option<String>,
}
