#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BatchEnvironmentConfigPeripheralsConfig {
    /// Resource name of an existing Dataproc Metastore service.
    #[builder(into)]
    #[serde(rename = "metastoreService")]
    pub r#metastore_service: Option<String>,
    /// The Spark History Server configuration for the workload.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "sparkHistoryServerConfig")]
    pub r#spark_history_server_config: Option<Box<super::super::types::dataproc::BatchEnvironmentConfigPeripheralsConfigSparkHistoryServerConfig>>,
}
