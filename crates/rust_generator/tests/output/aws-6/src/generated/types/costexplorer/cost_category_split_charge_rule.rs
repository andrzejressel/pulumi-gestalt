#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CostCategorySplitChargeRule {
    /// Method that's used to define how to split your source costs across your targets. Valid values are `FIXED`, `PROPORTIONAL`, `EVEN`
    #[builder(into)]
    #[serde(rename = "method")]
    pub r#method: String,
    /// Configuration block for the parameters for a split charge method. This is only required for the `FIXED` method. See below.
    #[builder(into)]
    #[serde(rename = "parameters")]
    pub r#parameters: Option<Vec<super::super::types::costexplorer::CostCategorySplitChargeRuleParameter>>,
    /// Cost Category value that you want to split.
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: String,
    /// Cost Category values that you want to split costs across. These values can't be used as a source in other split charge rules.
    #[builder(into)]
    #[serde(rename = "targets")]
    pub r#targets: Vec<String>,
}
