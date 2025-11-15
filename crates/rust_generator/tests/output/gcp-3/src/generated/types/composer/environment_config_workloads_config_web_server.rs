#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EnvironmentConfigWorkloadsConfigWebServer {
    /// CPU request and limit for Airflow web server.
    #[builder(into)]
    #[serde(rename = "cpu")]
    pub r#cpu: Option<f64>,
    /// Memory (GB) request and limit for Airflow web server.
    #[builder(into)]
    #[serde(rename = "memoryGb")]
    pub r#memory_gb: Option<f64>,
    /// Storage (GB) request and limit for Airflow web server.
    #[builder(into)]
    #[serde(rename = "storageGb")]
    pub r#storage_gb: Option<f64>,
}
