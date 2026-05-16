#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GuardrailContextualGroundingPolicyConfig {
    /// List of contextual grounding filter configs. See Contextual Grounding Filters Config for more information.
    #[builder(into)]
    #[serde(rename = "filtersConfigs")]
    pub r#filters_configs: Option<Vec<super::super::types::bedrock::GuardrailContextualGroundingPolicyConfigFiltersConfig>>,
}
