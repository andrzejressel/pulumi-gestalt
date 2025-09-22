#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AadDiagnosticSettingEnabledLog {
    /// The log category for the Azure Active Directory Diagnostic.
    #[builder(into)]
    #[serde(rename = "category")]
    pub r#category: String,
    #[builder(into)]
    #[serde(rename = "retentionPolicy")]
    pub r#retention_policy: Option<Box<super::super::types::monitoring::AadDiagnosticSettingEnabledLogRetentionPolicy>>,
}
