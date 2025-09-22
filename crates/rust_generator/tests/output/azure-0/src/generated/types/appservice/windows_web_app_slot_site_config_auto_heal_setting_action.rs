#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WindowsWebAppSlotSiteConfigAutoHealSettingAction {
    /// Predefined action to be taken to an Auto Heal trigger. Possible values are `CustomAction`, `LogEvent` and `Recycle`.
    #[builder(into)]
    #[serde(rename = "actionType")]
    pub r#action_type: String,
    /// A `custom_action` block as defined below.
    #[builder(into)]
    #[serde(rename = "customAction")]
    pub r#custom_action: Option<Box<super::super::types::appservice::WindowsWebAppSlotSiteConfigAutoHealSettingActionCustomAction>>,
    /// The minimum amount of time in `hh:mm:ss` the Windows Web App Slot must have been running before the defined action will be run in the event of a trigger.
    #[builder(into)]
    #[serde(rename = "minimumProcessExecutionTime")]
    pub r#minimum_process_execution_time: Option<String>,
}
