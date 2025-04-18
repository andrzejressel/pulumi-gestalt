#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetContactsRotationRecurrenceShiftCoverageCoverageTime {
    #[builder(into)]
    #[serde(rename = "ends")]
    pub r#ends: Box<Vec<super::super::types::ssm::GetContactsRotationRecurrenceShiftCoverageCoverageTimeEnd>>,
    #[builder(into)]
    #[serde(rename = "starts")]
    pub r#starts: Box<Vec<super::super::types::ssm::GetContactsRotationRecurrenceShiftCoverageCoverageTimeStart>>,
}
