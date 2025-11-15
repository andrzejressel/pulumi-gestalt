#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StreamSourceConfigSqlServerSourceConfigIncludeObjectsSchemaTable {
    /// SQL Server columns in the schema. When unspecified as part of include/exclude objects, includes/excludes everything.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "columns")]
    pub r#columns: Option<Vec<super::super::types::datastream::StreamSourceConfigSqlServerSourceConfigIncludeObjectsSchemaTableColumn>>,
    /// Table name.
    #[builder(into)]
    #[serde(rename = "table")]
    pub r#table: String,
}
