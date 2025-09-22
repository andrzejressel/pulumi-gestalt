#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EligibleRoleAssignmentSchedule {
    /// An `expiration` block as defined above.
    #[builder(into)]
    #[serde(rename = "expiration")]
    pub r#expiration: Option<Box<super::super::types::pim::EligibleRoleAssignmentScheduleExpiration>>,
    /// The start date/time of the role assignment. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "startDateTime")]
    pub r#start_date_time: Option<String>,
}
