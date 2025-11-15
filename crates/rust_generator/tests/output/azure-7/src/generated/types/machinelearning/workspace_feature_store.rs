#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WorkspaceFeatureStore {
    /// The version of Spark runtime.
    #[builder(into)]
    #[serde(rename = "computerSparkRuntimeVersion")]
    pub r#computer_spark_runtime_version: Option<String>,
    /// The name of offline store connection.
    #[builder(into)]
    #[serde(rename = "offlineConnectionName")]
    pub r#offline_connection_name: Option<String>,
    /// The name of online store connection.
    /// 
    /// > **Note:** `feature_store` must be set when`kind` is `FeatureStore`
    #[builder(into)]
    #[serde(rename = "onlineConnectionName")]
    pub r#online_connection_name: Option<String>,
}
