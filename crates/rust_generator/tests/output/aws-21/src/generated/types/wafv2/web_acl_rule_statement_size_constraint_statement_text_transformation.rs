#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WebAclRuleStatementSizeConstraintStatementTextTransformation {
    /// Relative processing order for multiple transformations that are defined for a rule statement. AWS WAF processes all transformations, from lowest priority to highest, before inspecting the transformed content.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: i32,
    /// Transformation to apply, please refer to the Text Transformation [documentation](https://docs.aws.amazon.com/waf/latest/APIReference/API_TextTransformation.html) for more details.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
