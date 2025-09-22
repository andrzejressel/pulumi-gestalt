#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetCostCategoryRuleRuleOr {
    /// Return results that match both `Dimension` objects.
    #[builder(into)]
    #[serde(rename = "ands")]
    pub r#ands: Vec<super::super::types::costexplorer::GetCostCategoryRuleRuleOrAnd>,
    /// Configuration block for the filter that's based on `CostCategory` values. See below.
    #[builder(into)]
    #[serde(rename = "costCategories")]
    pub r#cost_categories: Vec<super::super::types::costexplorer::GetCostCategoryRuleRuleOrCostCategory>,
    /// Configuration block for the specific `Dimension` to use for `Expression`. See below.
    #[builder(into)]
    #[serde(rename = "dimensions")]
    pub r#dimensions: Vec<super::super::types::costexplorer::GetCostCategoryRuleRuleOrDimension>,
    /// Return results that do not match the `Dimension` object.
    #[builder(into)]
    #[serde(rename = "nots")]
    pub r#nots: Vec<super::super::types::costexplorer::GetCostCategoryRuleRuleOrNot>,
    /// Return results that match either `Dimension` object.
    #[builder(into)]
    #[serde(rename = "ors")]
    pub r#ors: Vec<super::super::types::costexplorer::GetCostCategoryRuleRuleOrOr>,
    /// Configuration block for the specific `Tag` to use for `Expression`. See below.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Vec<super::super::types::costexplorer::GetCostCategoryRuleRuleOrTag>,
}
