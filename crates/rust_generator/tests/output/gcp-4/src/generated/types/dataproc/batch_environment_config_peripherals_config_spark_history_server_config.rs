#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BatchEnvironmentConfigPeripheralsConfigSparkHistoryServerConfig {
    /// Resource name of an existing Dataproc Cluster to act as a Spark History Server for the workload.
    #[builder(into)]
    #[serde(rename = "dataprocCluster")]
    pub r#dataproc_cluster: Option<String>,
}
