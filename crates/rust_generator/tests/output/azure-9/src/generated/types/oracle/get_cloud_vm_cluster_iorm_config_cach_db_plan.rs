#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetCloudVmClusterIormConfigCachDbPlan {
    /// The database name. For the default `DbPlan`, the `dbName` is `default`.
    #[builder(into)]
    #[serde(rename = "dbName")]
    pub r#db_name: String,
    /// The flash cache limit for this database. This value is internally configured based on the share value assigned to the database.
    #[builder(into)]
    #[serde(rename = "flashCacheLimit")]
    pub r#flash_cache_limit: String,
    /// The relative priority of this database.
    #[builder(into)]
    #[serde(rename = "share")]
    pub r#share: i32,
}
