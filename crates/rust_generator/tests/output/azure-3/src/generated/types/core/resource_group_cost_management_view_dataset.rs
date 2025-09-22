#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ResourceGroupCostManagementViewDataset {
    /// One or more `aggregation` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "aggregations")]
    pub r#aggregations: Vec<super::super::types::core::ResourceGroupCostManagementViewDatasetAggregation>,
    /// The granularity of rows in the report. Possible values are `Daily` and `Monthly`.
    #[builder(into)]
    #[serde(rename = "granularity")]
    pub r#granularity: String,
    /// One or more `grouping` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "groupings")]
    pub r#groupings: Option<Vec<super::super::types::core::ResourceGroupCostManagementViewDatasetGrouping>>,
    /// One or more `sorting` blocks as defined below, containing the order by expression to be used in the report
    #[builder(into)]
    #[serde(rename = "sortings")]
    pub r#sortings: Option<Vec<super::super::types::core::ResourceGroupCostManagementViewDatasetSorting>>,
}
