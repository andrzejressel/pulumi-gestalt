#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetTagsFilterNot {
    /// Configuration block for the filter that's based on `CostCategory` values. See `cost_category` block below for details.
    #[builder(into)]
    #[serde(rename = "costCategory")]
    pub r#cost_category: Option<Box<super::super::types::costexplorer::GetTagsFilterNotCostCategory>>,
    /// Configuration block for the specific `Dimension` to use for `Expression`. See `dimension` block below for details.
    #[builder(into)]
    #[serde(rename = "dimension")]
    pub r#dimension: Option<Box<super::super::types::costexplorer::GetTagsFilterNotDimension>>,
    /// Tags that match your request.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Option<Box<super::super::types::costexplorer::GetTagsFilterNotTags>>,
}
