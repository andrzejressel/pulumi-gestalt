#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApplicationApplicationConfigurationFlinkApplicationConfiguration {
    /// Describes an application's checkpointing configuration.
    #[builder(into)]
    #[serde(rename = "checkpointConfiguration")]
    pub r#checkpoint_configuration: Option<Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationFlinkApplicationConfigurationCheckpointConfiguration>>,
    /// Describes configuration parameters for CloudWatch logging for an application.
    #[builder(into)]
    #[serde(rename = "monitoringConfiguration")]
    pub r#monitoring_configuration: Option<Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationFlinkApplicationConfigurationMonitoringConfiguration>>,
    /// Describes parameters for how an application executes multiple tasks simultaneously.
    #[builder(into)]
    #[serde(rename = "parallelismConfiguration")]
    pub r#parallelism_configuration: Option<Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationFlinkApplicationConfigurationParallelismConfiguration>>,
}
