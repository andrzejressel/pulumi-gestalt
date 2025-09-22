#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionDiscoveryConfigTargetCloudSqlTargetConditions {
    /// Database engines that should be profiled. Optional. Defaults to ALL_SUPPORTED_DATABASE_ENGINES if unspecified.
    /// Each value may be one of: `ALL_SUPPORTED_DATABASE_ENGINES`, `MYSQL`, `POSTGRES`.
    #[builder(into)]
    #[serde(rename = "databaseEngines")]
    pub r#database_engines: Option<Vec<String>>,
    /// Data profiles will only be generated for the database resource types specified in this field. If not specified, defaults to [DATABASE_RESOURCE_TYPE_ALL_SUPPORTED_TYPES].
    /// Each value may be one of: `DATABASE_RESOURCE_TYPE_ALL_SUPPORTED_TYPES`, `DATABASE_RESOURCE_TYPE_TABLE`.
    #[builder(into)]
    #[serde(rename = "types")]
    pub r#types: Option<Vec<String>>,
}
