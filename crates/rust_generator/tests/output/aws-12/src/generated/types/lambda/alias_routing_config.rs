#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AliasRoutingConfig {
    /// A map that defines the proportion of events that should be sent to different versions of a lambda function.
    #[builder(into)]
    #[serde(rename = "additionalVersionWeights")]
    pub r#additional_version_weights: Option<std::collections::HashMap<String, f64>>,
}
