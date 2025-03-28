#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct IngestionDestinationProcessingConfigurationAuditLog {
    /// The format in which the audit logs need to be formatted. Valid values: `json`, `parquet`.
    #[builder(into)]
    #[serde(rename = "format")]
    pub r#format: Box<String>,
    /// The event schema in which the audit logs need to be formatted. Valid values: `ocsf`, `raw`.
    #[builder(into)]
    #[serde(rename = "schema")]
    pub r#schema: Box<String>,
}
