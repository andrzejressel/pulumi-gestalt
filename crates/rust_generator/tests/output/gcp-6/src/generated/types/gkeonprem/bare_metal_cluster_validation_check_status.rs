#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BareMetalClusterValidationCheckStatus {
    /// (Output)
    /// Individual checks which failed as part of the Preflight check execution.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "results")]
    pub r#results: Option<Vec<super::super::types::gkeonprem::BareMetalClusterValidationCheckStatusResult>>,
}
