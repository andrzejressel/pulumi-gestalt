#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ScalingPlanApplicationSource {
    /// ARN of a AWS CloudFormation stack.
    #[builder(into)]
    #[serde(rename = "cloudformationStackArn")]
    pub r#cloudformation_stack_arn: Option<String>,
    /// Set of tags.
    #[builder(into)]
    #[serde(rename = "tagFilters")]
    pub r#tag_filters: Option<Vec<super::super::types::autoscalingplans::ScalingPlanApplicationSourceTagFilter>>,
}
