#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceHealthCheckConfig {
    /// The number of consecutive health checks. Maximum value of 10.
    #[builder(into)]
    #[serde(rename = "failureThreshold")]
    pub r#failure_threshold: Option<i32>,
    /// The path that you want Route 53 to request when performing health checks. Route 53 automatically adds the DNS name for the service. If you don't specify a value, the default value is /.
    #[builder(into)]
    #[serde(rename = "resourcePath")]
    pub r#resource_path: Option<String>,
    /// The type of health check that you want to create, which indicates how Route 53 determines whether an endpoint is healthy. Valid Values: HTTP, HTTPS, TCP
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
}
