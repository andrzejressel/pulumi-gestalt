#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BudgetManagementGroupTimePeriod {
    /// The end date for the budget. If not set this will be 10 years after the start date.
    #[builder(into)]
    #[serde(rename = "endDate")]
    pub r#end_date: Option<String>,
    /// The start date for the budget. The start date must be first of the month and should be less than the end date. Budget start date must be on or after June 1, 2017. Future start date should not be more than twelve months. Past start date should be selected within the timegrain period. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "startDate")]
    pub r#start_date: String,
}
