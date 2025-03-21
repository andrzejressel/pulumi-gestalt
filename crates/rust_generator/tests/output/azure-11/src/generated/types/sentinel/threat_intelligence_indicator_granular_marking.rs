#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ThreatIntelligenceIndicatorGranularMarking {
    /// The language of granular marking of the Threat Intelligence Indicator.
    #[builder(into, default)]
    #[serde(rename = "language")]
    pub r#language: Box<Option<String>>,
    /// The reference of the granular marking of the Threat Intelligence Indicator.
    #[builder(into, default)]
    #[serde(rename = "markingRef")]
    pub r#marking_ref: Box<Option<String>>,
    /// A list of selectors of the granular marking of the Threat Intelligence Indicator.
    #[builder(into, default)]
    #[serde(rename = "selectors")]
    pub r#selectors: Box<Option<Vec<String>>>,
}
