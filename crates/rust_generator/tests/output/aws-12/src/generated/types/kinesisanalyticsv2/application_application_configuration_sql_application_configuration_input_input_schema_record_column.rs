#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApplicationApplicationConfigurationSqlApplicationConfigurationInputInputSchemaRecordColumn {
    /// A reference to the data element in the streaming input or the reference data source.
    #[builder(into)]
    #[serde(rename = "mapping")]
    pub r#mapping: Option<String>,
    /// The name of the column that is created in the in-application input stream or reference table.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The type of column created in the in-application input stream or reference table.
    #[builder(into)]
    #[serde(rename = "sqlType")]
    pub r#sql_type: String,
}
