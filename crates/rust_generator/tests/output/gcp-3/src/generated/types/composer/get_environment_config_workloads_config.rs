#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetEnvironmentConfigWorkloadsConfig {
    /// Configuration for resources used by DAG processor.
    #[builder(into)]
    #[serde(rename = "dagProcessors")]
    pub r#dag_processors: Vec<super::super::types::composer::GetEnvironmentConfigWorkloadsConfigDagProcessor>,
    /// Configuration for resources used by Airflow schedulers.
    #[builder(into)]
    #[serde(rename = "schedulers")]
    pub r#schedulers: Vec<super::super::types::composer::GetEnvironmentConfigWorkloadsConfigScheduler>,
    /// Configuration for resources used by Airflow triggerers.
    #[builder(into)]
    #[serde(rename = "triggerers")]
    pub r#triggerers: Vec<super::super::types::composer::GetEnvironmentConfigWorkloadsConfigTriggerer>,
    /// Configuration for resources used by Airflow web server.
    #[builder(into)]
    #[serde(rename = "webServers")]
    pub r#web_servers: Vec<super::super::types::composer::GetEnvironmentConfigWorkloadsConfigWebServer>,
    /// Configuration for resources used by Airflow workers.
    #[builder(into)]
    #[serde(rename = "workers")]
    pub r#workers: Vec<super::super::types::composer::GetEnvironmentConfigWorkloadsConfigWorker>,
}
