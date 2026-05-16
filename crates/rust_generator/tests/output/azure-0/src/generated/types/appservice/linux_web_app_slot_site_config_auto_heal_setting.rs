#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LinuxWebAppSlotSiteConfigAutoHealSetting {
    /// A `action` block as defined above.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Option<Box<super::super::types::appservice::LinuxWebAppSlotSiteConfigAutoHealSettingAction>>,
    /// A `trigger` block as defined below.
    #[builder(into)]
    #[serde(rename = "trigger")]
    pub r#trigger: Option<Box<super::super::types::appservice::LinuxWebAppSlotSiteConfigAutoHealSettingTrigger>>,
}
