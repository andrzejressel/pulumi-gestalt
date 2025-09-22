#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct MonitoringScheduleMonitoringScheduleConfig {
    /// The name of the monitoring job definition to schedule.
    #[builder(into)]
    #[serde(rename = "monitoringJobDefinitionName")]
    pub r#monitoring_job_definition_name: String,
    /// The type of the monitoring job definition to schedule. Valid values are `DataQuality`, `ModelQuality`, `ModelBias` or `ModelExplainability`
    #[builder(into)]
    #[serde(rename = "monitoringType")]
    pub r#monitoring_type: String,
    /// Configures the monitoring schedule. Fields are documented below.
    #[builder(into)]
    #[serde(rename = "scheduleConfig")]
    pub r#schedule_config: Option<Box<super::super::types::sagemaker::MonitoringScheduleMonitoringScheduleConfigScheduleConfig>>,
}
