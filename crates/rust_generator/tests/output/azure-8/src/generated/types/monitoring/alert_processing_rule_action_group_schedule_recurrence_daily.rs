#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AlertProcessingRuleActionGroupScheduleRecurrenceDaily {
    /// Specifies the recurrence end time (H:M:S).
    #[builder(into)]
    #[serde(rename = "endTime")]
    pub r#end_time: String,
    /// Specifies the recurrence start time (H:M:S).
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: String,
}
