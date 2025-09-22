#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AutomationRuleActionFindingFieldsUpdateRelatedFinding {
    /// The product-generated identifier for a related finding.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// The ARN of the product that generated a related finding.
    #[builder(into)]
    #[serde(rename = "productArn")]
    pub r#product_arn: String,
}
