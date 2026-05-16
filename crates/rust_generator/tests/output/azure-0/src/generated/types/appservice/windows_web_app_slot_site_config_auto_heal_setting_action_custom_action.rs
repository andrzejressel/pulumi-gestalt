#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WindowsWebAppSlotSiteConfigAutoHealSettingActionCustomAction {
    /// The executable to run for the `custom_action`.
    #[builder(into)]
    #[serde(rename = "executable")]
    pub r#executable: String,
    /// The parameters to pass to the specified `executable`.
    #[builder(into)]
    #[serde(rename = "parameters")]
    pub r#parameters: Option<String>,
}
