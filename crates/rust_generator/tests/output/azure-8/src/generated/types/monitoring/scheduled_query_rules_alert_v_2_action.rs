#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ScheduledQueryRulesAlertV2Action {
    /// List of Action Group resource IDs to invoke when the alert fires.
    #[builder(into)]
    #[serde(rename = "actionGroups")]
    pub r#action_groups: Option<Vec<String>>,
    /// Specifies the properties of an alert payload.
    #[builder(into)]
    #[serde(rename = "customProperties")]
    pub r#custom_properties: Option<std::collections::HashMap<String, String>>,
}
