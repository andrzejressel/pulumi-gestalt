#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ContactsRotationRecurrenceShiftCoverageCoverageTime {
    /// (Required) The end time of the on-call shift. See Hand Off Time for more details.
    #[builder(into)]
    #[serde(rename = "end")]
    pub r#end: Option<Box<super::super::types::ssm::ContactsRotationRecurrenceShiftCoverageCoverageTimeEnd>>,
    /// (Required) The start time of the on-call shift. See Hand Off Time for more details.
    #[builder(into)]
    #[serde(rename = "start")]
    pub r#start: Option<Box<super::super::types::ssm::ContactsRotationRecurrenceShiftCoverageCoverageTimeStart>>,
}
