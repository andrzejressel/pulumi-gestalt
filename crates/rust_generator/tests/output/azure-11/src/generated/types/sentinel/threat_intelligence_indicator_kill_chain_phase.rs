#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ThreatIntelligenceIndicatorKillChainPhase {
    /// The name which should be used for the Lockheed Martin cyber kill chain phase.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
}
