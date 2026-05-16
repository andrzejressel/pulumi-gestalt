#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetInfrastructureConfigurationLogging {
    /// Nested list of S3 logs settings.
    #[builder(into)]
    #[serde(rename = "s3Logs")]
    pub r#s_3_logs: Vec<super::super::types::imagebuilder::GetInfrastructureConfigurationLoggingS3Log>,
}
