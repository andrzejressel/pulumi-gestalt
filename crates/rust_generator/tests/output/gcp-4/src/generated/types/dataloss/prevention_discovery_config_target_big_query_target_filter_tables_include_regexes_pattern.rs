#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionDiscoveryConfigTargetBigQueryTargetFilterTablesIncludeRegexesPattern {
    /// if unset, this property matches all datasets
    #[builder(into)]
    #[serde(rename = "datasetIdRegex")]
    pub r#dataset_id_regex: Option<String>,
    /// For organizations, if unset, will match all projects. Has no effect for data profile configurations created within a project.
    #[builder(into)]
    #[serde(rename = "projectIdRegex")]
    pub r#project_id_regex: Option<String>,
    /// if unset, this property matches all tables
    #[builder(into)]
    #[serde(rename = "tableIdRegex")]
    pub r#table_id_regex: Option<String>,
}
