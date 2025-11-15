#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ThreatIntelligenceIndicatorParsedPatternPatternTypeValue {
    /// The value of the parsed pattern type.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Option<String>,
    /// The type of the value of the parsed pattern type value.
    #[builder(into)]
    #[serde(rename = "valueType")]
    pub r#value_type: Option<String>,
}
