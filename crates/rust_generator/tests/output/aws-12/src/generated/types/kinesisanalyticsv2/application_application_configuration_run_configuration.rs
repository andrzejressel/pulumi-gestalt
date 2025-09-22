#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ApplicationApplicationConfigurationRunConfiguration {
    /// The restore behavior of a restarting application.
    #[builder(into)]
    #[serde(rename = "applicationRestoreConfiguration")]
    pub r#application_restore_configuration: Option<Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationRunConfigurationApplicationRestoreConfiguration>>,
    /// The starting parameters for a Flink-based Kinesis Data Analytics application.
    #[builder(into)]
    #[serde(rename = "flinkRunConfiguration")]
    pub r#flink_run_configuration: Option<Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationRunConfigurationFlinkRunConfiguration>>,
}
