#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EnvironmentConfigWorkloadsConfigScheduler {
    /// The number of schedulers.
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: Option<i32>,
    /// CPU request and limit for a single Airflow scheduler replica
    #[builder(into)]
    #[serde(rename = "cpu")]
    pub r#cpu: Option<f64>,
    /// Memory (GB) request and limit for a single Airflow scheduler replica.
    #[builder(into)]
    #[serde(rename = "memoryGb")]
    pub r#memory_gb: Option<f64>,
    /// Storage (GB) request and limit for a single Airflow scheduler replica.
    #[builder(into)]
    #[serde(rename = "storageGb")]
    pub r#storage_gb: Option<f64>,
}
