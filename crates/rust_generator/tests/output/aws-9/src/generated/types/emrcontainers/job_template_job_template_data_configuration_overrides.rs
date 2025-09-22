#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobTemplateJobTemplateDataConfigurationOverrides {
    /// The configurations for the application running by the job run.
    #[builder(into)]
    #[serde(rename = "applicationConfigurations")]
    pub r#application_configurations: Option<Vec<super::super::types::emrcontainers::JobTemplateJobTemplateDataConfigurationOverridesApplicationConfiguration>>,
    /// The configurations for monitoring.
    #[builder(into)]
    #[serde(rename = "monitoringConfiguration")]
    pub r#monitoring_configuration: Option<Box<super::super::types::emrcontainers::JobTemplateJobTemplateDataConfigurationOverridesMonitoringConfiguration>>,
}
