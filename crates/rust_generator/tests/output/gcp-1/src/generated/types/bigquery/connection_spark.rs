#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConnectionSpark {
    /// Dataproc Metastore Service configuration for the connection.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "metastoreServiceConfig")]
    pub r#metastore_service_config: Option<Box<super::super::types::bigquery::ConnectionSparkMetastoreServiceConfig>>,
    /// (Output)
    /// The account ID of the service created for the purpose of this connection.
    #[builder(into)]
    #[serde(rename = "serviceAccountId")]
    pub r#service_account_id: Option<String>,
    /// Spark History Server configuration for the connection.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "sparkHistoryServerConfig")]
    pub r#spark_history_server_config: Option<Box<super::super::types::bigquery::ConnectionSparkSparkHistoryServerConfig>>,
}
