#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ThreatIntelligenceIndicatorGranularMarking {
    /// The language of granular marking of the Threat Intelligence Indicator.
    #[builder(into)]
    #[serde(rename = "language")]
    pub r#language: Option<String>,
    /// The reference of the granular marking of the Threat Intelligence Indicator.
    #[builder(into)]
    #[serde(rename = "markingRef")]
    pub r#marking_ref: Option<String>,
    /// A list of selectors of the granular marking of the Threat Intelligence Indicator.
    #[builder(into)]
    #[serde(rename = "selectors")]
    pub r#selectors: Option<Vec<String>>,
}
