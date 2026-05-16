#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ThreatIntelligenceIndicatorParsedPattern {
    /// The type key of parsed pattern.
    #[builder(into)]
    #[serde(rename = "patternTypeKey")]
    pub r#pattern_type_key: Option<String>,
    /// A `pattern_type_values` block as defined below.
    #[builder(into)]
    #[serde(rename = "patternTypeValues")]
    pub r#pattern_type_values: Option<Vec<super::super::types::sentinel::ThreatIntelligenceIndicatorParsedPatternPatternTypeValue>>,
}
