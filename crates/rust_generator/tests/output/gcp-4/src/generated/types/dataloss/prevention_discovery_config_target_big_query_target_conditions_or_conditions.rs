#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionDiscoveryConfigTargetBigQueryTargetConditionsOrConditions {
    /// Duration format. The minimum age a table must have before Cloud DLP can profile it. Value greater than 1.
    #[builder(into)]
    #[serde(rename = "minAge")]
    pub r#min_age: Option<String>,
    /// Minimum number of rows that should be present before Cloud DLP profiles as a table.
    #[builder(into)]
    #[serde(rename = "minRowCount")]
    pub r#min_row_count: Option<i32>,
}
