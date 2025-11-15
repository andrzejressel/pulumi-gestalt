#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetLoadBalancerAccessLogs {
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: String,
    #[builder(into)]
    #[serde(rename = "bucketPrefix")]
    pub r#bucket_prefix: String,
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: bool,
    #[builder(into)]
    #[serde(rename = "interval")]
    pub r#interval: i32,
}
