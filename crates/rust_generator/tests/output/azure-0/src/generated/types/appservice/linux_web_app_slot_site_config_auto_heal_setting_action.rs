#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LinuxWebAppSlotSiteConfigAutoHealSettingAction {
    /// Predefined action to be taken to an Auto Heal trigger. Possible values include: `Recycle`.
    #[builder(into)]
    #[serde(rename = "actionType")]
    pub r#action_type: String,
    /// The minimum amount of time in `hh:mm:ss` the Linux Web App must have been running before the defined action will be run in the event of a trigger.
    #[builder(into)]
    #[serde(rename = "minimumProcessExecutionTime")]
    pub r#minimum_process_execution_time: Option<String>,
}
