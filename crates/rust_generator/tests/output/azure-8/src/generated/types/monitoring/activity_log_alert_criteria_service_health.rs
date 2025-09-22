#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ActivityLogAlertCriteriaServiceHealth {
    /// Events this alert will monitor Possible values are `Incident`, `Maintenance`, `Informational`, `ActionRequired` and `Security`.
    #[builder(into)]
    #[serde(rename = "events")]
    pub r#events: Option<Vec<String>>,
    /// Locations this alert will monitor. For example, `West Europe`.
    #[builder(into)]
    #[serde(rename = "locations")]
    pub r#locations: Option<Vec<String>>,
    /// Services this alert will monitor. For example, `Activity Logs & Alerts`, `Action Groups`. Defaults to all Services.
    #[builder(into)]
    #[serde(rename = "services")]
    pub r#services: Option<Vec<String>>,
}
