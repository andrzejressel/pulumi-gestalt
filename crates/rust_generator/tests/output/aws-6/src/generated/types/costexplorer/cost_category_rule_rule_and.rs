#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CostCategoryRuleRuleAnd {
    /// Return results that match both `Dimension` objects.
    #[builder(into)]
    #[serde(rename = "ands")]
    pub r#ands: Option<Vec<super::super::types::costexplorer::CostCategoryRuleRuleAndAnd>>,
    /// Configuration block for the filter that's based on `CostCategory` values. See below.
    #[builder(into)]
    #[serde(rename = "costCategory")]
    pub r#cost_category: Option<Box<super::super::types::costexplorer::CostCategoryRuleRuleAndCostCategory>>,
    /// Configuration block for the specific `Dimension` to use for `Expression`. See below.
    #[builder(into)]
    #[serde(rename = "dimension")]
    pub r#dimension: Option<Box<super::super::types::costexplorer::CostCategoryRuleRuleAndDimension>>,
    /// Return results that match both `Dimension` object.
    #[builder(into)]
    #[serde(rename = "not")]
    pub r#not: Option<Box<super::super::types::costexplorer::CostCategoryRuleRuleAndNot>>,
    /// Return results that match both `Dimension` object.
    #[builder(into)]
    #[serde(rename = "ors")]
    pub r#ors: Option<Vec<super::super::types::costexplorer::CostCategoryRuleRuleAndOr>>,
    /// Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Option<Box<super::super::types::costexplorer::CostCategoryRuleRuleAndTags>>,
}
