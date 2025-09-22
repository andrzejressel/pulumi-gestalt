#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CostCategoryRuleRuleOr {
    /// Return results that match both `Dimension` objects.
    #[builder(into)]
    #[serde(rename = "ands")]
    pub r#ands: Option<Vec<super::super::types::costexplorer::CostCategoryRuleRuleOrAnd>>,
    /// Configuration block for the filter that's based on `CostCategory` values. See below.
    #[builder(into)]
    #[serde(rename = "costCategory")]
    pub r#cost_category: Box<Option<super::super::types::costexplorer::CostCategoryRuleRuleOrCostCategory>>,
    /// Configuration block for the specific `Dimension` to use for `Expression`. See below.
    #[builder(into)]
    #[serde(rename = "dimension")]
    pub r#dimension: Box<Option<super::super::types::costexplorer::CostCategoryRuleRuleOrDimension>>,
    /// Return results that match both `Dimension` object.
    #[builder(into)]
    #[serde(rename = "not")]
    pub r#not: Box<Option<super::super::types::costexplorer::CostCategoryRuleRuleOrNot>>,
    /// Return results that match both `Dimension` object.
    #[builder(into)]
    #[serde(rename = "ors")]
    pub r#ors: Option<Vec<super::super::types::costexplorer::CostCategoryRuleRuleOrOr>>,
    /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Option<super::super::types::costexplorer::CostCategoryRuleRuleOrTags>>,
}
