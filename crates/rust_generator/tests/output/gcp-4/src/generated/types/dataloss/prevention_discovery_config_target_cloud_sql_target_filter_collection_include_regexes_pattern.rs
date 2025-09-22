#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionDiscoveryConfigTargetCloudSqlTargetFilterCollectionIncludeRegexesPattern {
    /// Regex to test the database name against. If empty, all databases match.
    #[builder(into)]
    #[serde(rename = "databaseRegex")]
    pub r#database_regex: Option<String>,
    /// Regex to test the database resource's name against. An example of a database resource name is a table's name. Other database resource names like view names could be included in the future. If empty, all database resources match.'
    #[builder(into)]
    #[serde(rename = "databaseResourceNameRegex")]
    pub r#database_resource_name_regex: Option<String>,
    /// Regex to test the instance name against. If empty, all instances match.
    #[builder(into)]
    #[serde(rename = "instanceRegex")]
    pub r#instance_regex: Option<String>,
    /// For organizations, if unset, will match all projects. Has no effect for data profile configurations created within a project.
    #[builder(into)]
    #[serde(rename = "projectIdRegex")]
    pub r#project_id_regex: Option<String>,
}
