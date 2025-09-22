#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetEndpointElasticsearchSetting {
    #[builder(into)]
    #[serde(rename = "endpointUri")]
    pub r#endpoint_uri: String,
    #[builder(into)]
    #[serde(rename = "errorRetryDuration")]
    pub r#error_retry_duration: i32,
    #[builder(into)]
    #[serde(rename = "fullLoadErrorPercentage")]
    pub r#full_load_error_percentage: i32,
    #[builder(into)]
    #[serde(rename = "serviceAccessRoleArn")]
    pub r#service_access_role_arn: String,
}
