#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetListenerDefaultActionForward {
    #[builder(into)]
    #[serde(rename = "stickinesses")]
    pub r#stickinesses: Vec<super::super::types::alb::GetListenerDefaultActionForwardStickiness>,
    #[builder(into)]
    #[serde(rename = "targetGroups")]
    pub r#target_groups: Vec<super::super::types::alb::GetListenerDefaultActionForwardTargetGroup>,
}
