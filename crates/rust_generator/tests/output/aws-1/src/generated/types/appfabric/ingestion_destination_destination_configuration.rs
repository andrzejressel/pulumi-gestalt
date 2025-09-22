#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct IngestionDestinationDestinationConfiguration {
    /// Contains information about an audit log processing configuration.
    #[builder(into)]
    #[serde(rename = "auditLog")]
    pub r#audit_log: Option<Box<super::super::types::appfabric::IngestionDestinationDestinationConfigurationAuditLog>>,
}
