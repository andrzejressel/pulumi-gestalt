#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ExportExportDataQuery {
    /// Query statement.
    #[builder(into)]
    #[serde(rename = "queryStatement")]
    pub r#query_statement: String,
    /// Table configuration.
    #[builder(into)]
    #[serde(rename = "tableConfigurations")]
    pub r#table_configurations: Option<std::collections::HashMap<String, std::collections::HashMap<String, String>>>,
}
