#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AnalyzerConfiguration {
    /// A block that specifies the configuration of an unused access analyzer for an AWS organization or account. Documented below
    #[builder(into)]
    #[serde(rename = "unusedAccess")]
    pub r#unused_access: Option<Box<super::super::types::accessanalyzer::AnalyzerConfigurationUnusedAccess>>,
}
