#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct IngestionDestinationProcessingConfiguration {
    /// Contains information about an audit log processing configuration.
    #[builder(into)]
    #[serde(rename = "auditLog")]
    pub r#audit_log: Option<Box<super::super::types::appfabric::IngestionDestinationProcessingConfigurationAuditLog>>,
}
