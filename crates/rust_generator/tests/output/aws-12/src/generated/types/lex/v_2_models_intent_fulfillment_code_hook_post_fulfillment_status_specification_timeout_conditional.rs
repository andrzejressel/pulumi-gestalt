#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct V2ModelsIntentFulfillmentCodeHookPostFulfillmentStatusSpecificationTimeoutConditional {
    /// Whether a conditional branch is active. When active is false, the conditions are not evaluated.
    #[builder(into)]
    #[serde(rename = "active")]
    pub r#active: bool,
    /// Configuration blocks for conditional branches. A conditional branch is made up of a condition, a response and a next step. The response and next step are executed when the condition is true. See `conditional_branch`.
    #[builder(into)]
    #[serde(rename = "conditionalBranches")]
    pub r#conditional_branches: Option<Vec<super::super::types::lex::V2ModelsIntentFulfillmentCodeHookPostFulfillmentStatusSpecificationTimeoutConditionalConditionalBranch>>,
    /// Configuration block for the conditional branch that should be followed when the conditions for other branches are not satisfied. A branch is made up of a condition, a response and a next step. See `default_branch`.
    #[builder(into)]
    #[serde(rename = "defaultBranch")]
    pub r#default_branch: Box<super::super::types::lex::V2ModelsIntentFulfillmentCodeHookPostFulfillmentStatusSpecificationTimeoutConditionalDefaultBranch>,
}
