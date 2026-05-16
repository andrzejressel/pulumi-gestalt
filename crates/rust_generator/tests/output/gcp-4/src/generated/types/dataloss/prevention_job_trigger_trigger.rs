#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionJobTriggerTrigger {
    /// For use with hybrid jobs. Jobs must be manually created and finished.
    #[builder(into)]
    #[serde(rename = "manual")]
    pub r#manual: Option<Box<super::super::types::dataloss::PreventionJobTriggerTriggerManual>>,
    /// Schedule for triggered jobs
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "schedule")]
    pub r#schedule: Option<Box<super::super::types::dataloss::PreventionJobTriggerTriggerSchedule>>,
}
