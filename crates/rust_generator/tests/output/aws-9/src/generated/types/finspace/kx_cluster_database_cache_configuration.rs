#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct KxClusterDatabaseCacheConfiguration {
    /// Type of disk cache.
    #[builder(into)]
    #[serde(rename = "cacheType")]
    pub r#cache_type: String,
    /// Paths within the database to cache.
    #[builder(into)]
    #[serde(rename = "dbPaths")]
    pub r#db_paths: Option<Vec<String>>,
}
