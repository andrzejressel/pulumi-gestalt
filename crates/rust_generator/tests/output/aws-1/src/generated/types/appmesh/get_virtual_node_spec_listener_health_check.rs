#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetVirtualNodeSpecListenerHealthCheck {
    #[builder(into)]
    #[serde(rename = "healthyThreshold")]
    pub r#healthy_threshold: i32,
    #[builder(into)]
    #[serde(rename = "intervalMillis")]
    pub r#interval_millis: i32,
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: String,
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: i32,
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: String,
    #[builder(into)]
    #[serde(rename = "timeoutMillis")]
    pub r#timeout_millis: i32,
    #[builder(into)]
    #[serde(rename = "unhealthyThreshold")]
    pub r#unhealthy_threshold: i32,
}
