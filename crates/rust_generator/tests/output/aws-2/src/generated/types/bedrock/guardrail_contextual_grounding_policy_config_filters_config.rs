#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GuardrailContextualGroundingPolicyConfigFiltersConfig {
    /// The threshold for this filter.
    #[builder(into)]
    #[serde(rename = "threshold")]
    pub r#threshold: f64,
    /// Type of contextual grounding filter.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
