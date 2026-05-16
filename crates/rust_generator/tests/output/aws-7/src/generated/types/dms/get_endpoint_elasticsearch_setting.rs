#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
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
