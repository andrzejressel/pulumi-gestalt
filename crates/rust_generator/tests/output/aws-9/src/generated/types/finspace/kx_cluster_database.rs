#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KxClusterDatabase {
    /// Configuration details for the disk cache to increase performance reading from a KX database mounted to the cluster. See cache_configurations.
    #[builder(into)]
    #[serde(rename = "cacheConfigurations")]
    pub r#cache_configurations: Option<Vec<super::super::types::finspace::KxClusterDatabaseCacheConfiguration>>,
    /// A unique identifier of the changeset that is associated with the cluster.
    #[builder(into)]
    #[serde(rename = "changesetId")]
    pub r#changeset_id: Option<String>,
    /// Name of the KX database.
    #[builder(into)]
    #[serde(rename = "databaseName")]
    pub r#database_name: String,
    /// The name of the dataview to be used for caching historical data on disk. You cannot update to a different dataview name once a cluster is created. Use `lifecycle` `ignore_changes` for database to prevent any undesirable behaviors.
    #[builder(into)]
    #[serde(rename = "dataviewName")]
    pub r#dataview_name: Option<String>,
}
