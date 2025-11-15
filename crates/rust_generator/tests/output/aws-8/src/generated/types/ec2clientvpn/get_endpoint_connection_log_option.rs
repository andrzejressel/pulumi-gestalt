#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetEndpointConnectionLogOption {
    #[builder(into)]
    #[serde(rename = "cloudwatchLogGroup")]
    pub r#cloudwatch_log_group: String,
    #[builder(into)]
    #[serde(rename = "cloudwatchLogStream")]
    pub r#cloudwatch_log_stream: String,
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
}
