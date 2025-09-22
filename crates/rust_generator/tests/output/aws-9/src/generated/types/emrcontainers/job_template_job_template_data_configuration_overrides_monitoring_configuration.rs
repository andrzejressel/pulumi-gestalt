#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobTemplateJobTemplateDataConfigurationOverridesMonitoringConfiguration {
    /// Monitoring configurations for CloudWatch.
    #[builder(into)]
    #[serde(rename = "cloudWatchMonitoringConfiguration")]
    pub r#cloud_watch_monitoring_configuration: Box<Option<super::super::types::emrcontainers::JobTemplateJobTemplateDataConfigurationOverridesMonitoringConfigurationCloudWatchMonitoringConfiguration>>,
    /// Monitoring configurations for the persistent application UI.
    #[builder(into)]
    #[serde(rename = "persistentAppUi")]
    pub r#persistent_app_ui: Option<String>,
    /// Amazon S3 configuration for monitoring log publishing.
    #[builder(into)]
    #[serde(rename = "s3MonitoringConfiguration")]
    pub r#s_3_monitoring_configuration: Box<Option<super::super::types::emrcontainers::JobTemplateJobTemplateDataConfigurationOverridesMonitoringConfigurationS3MonitoringConfiguration>>,
}
