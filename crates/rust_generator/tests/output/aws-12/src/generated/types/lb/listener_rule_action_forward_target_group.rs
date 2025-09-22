#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ListenerRuleActionForwardTargetGroup {
    /// The Amazon Resource Name (ARN) of the target group.
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: String,
    /// The weight. The range is 0 to 999.
    #[builder(into)]
    #[serde(rename = "weight")]
    pub r#weight: Option<i32>,
}
