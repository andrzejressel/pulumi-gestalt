#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EnvironmentConfigWorkloadsConfig {
    /// Configuration for resources used by DAG processor.
    #[builder(into)]
    #[serde(rename = "dagProcessor")]
    pub r#dag_processor: Option<Box<super::super::types::composer::EnvironmentConfigWorkloadsConfigDagProcessor>>,
    /// Configuration for resources used by Airflow schedulers.
    #[builder(into)]
    #[serde(rename = "scheduler")]
    pub r#scheduler: Option<Box<super::super::types::composer::EnvironmentConfigWorkloadsConfigScheduler>>,
    /// Configuration for resources used by Airflow triggerers.
    #[builder(into)]
    #[serde(rename = "triggerer")]
    pub r#triggerer: Option<Box<super::super::types::composer::EnvironmentConfigWorkloadsConfigTriggerer>>,
    /// Configuration for resources used by Airflow web server.
    #[builder(into)]
    #[serde(rename = "webServer")]
    pub r#web_server: Option<Box<super::super::types::composer::EnvironmentConfigWorkloadsConfigWebServer>>,
    /// Configuration for resources used by Airflow workers.
    #[builder(into)]
    #[serde(rename = "worker")]
    pub r#worker: Option<Box<super::super::types::composer::EnvironmentConfigWorkloadsConfigWorker>>,
}
