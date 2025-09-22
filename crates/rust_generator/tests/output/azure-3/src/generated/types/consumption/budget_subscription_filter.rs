#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BudgetSubscriptionFilter {
    /// One or more `dimension` blocks as defined below to filter the budget on.
    #[builder(into)]
    #[serde(rename = "dimensions")]
    pub r#dimensions: Option<Vec<super::super::types::consumption::BudgetSubscriptionFilterDimension>>,
    /// One or more `tag` blocks as defined below to filter the budget on.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Option<Vec<super::super::types::consumption::BudgetSubscriptionFilterTag>>,
}
