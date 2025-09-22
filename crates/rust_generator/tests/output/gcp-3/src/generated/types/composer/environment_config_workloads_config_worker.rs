#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EnvironmentConfigWorkloadsConfigWorker {
    /// CPU request and limit for a single Airflow worker replica.
    #[builder(into)]
    #[serde(rename = "cpu")]
    pub r#cpu: Option<f64>,
    /// Maximum number of workers for autoscaling.
    #[builder(into)]
    #[serde(rename = "maxCount")]
    pub r#max_count: Option<i32>,
    /// Memory (GB) request and limit for a single Airflow worker replica.
    #[builder(into)]
    #[serde(rename = "memoryGb")]
    pub r#memory_gb: Option<f64>,
    /// Minimum number of workers for autoscaling.
    #[builder(into)]
    #[serde(rename = "minCount")]
    pub r#min_count: Option<i32>,
    /// Storage (GB) request and limit for a single Airflow worker replica.
    #[builder(into)]
    #[serde(rename = "storageGb")]
    pub r#storage_gb: Option<f64>,
}
