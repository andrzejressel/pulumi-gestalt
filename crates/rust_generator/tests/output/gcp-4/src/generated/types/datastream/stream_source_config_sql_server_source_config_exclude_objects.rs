#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct StreamSourceConfigSqlServerSourceConfigExcludeObjects {
    /// SQL Server schemas/databases in the database server
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "schemas")]
    pub r#schemas: Box<Vec<super::super::types::datastream::StreamSourceConfigSqlServerSourceConfigExcludeObjectsSchema>>,
}
