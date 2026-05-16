#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BotIntent {
    /// The name of the intent. Must be less than or equal to 100 characters in length.
    #[builder(into)]
    #[serde(rename = "intentName")]
    pub r#intent_name: String,
    /// The version of the intent. Must be less than or equal to 64 characters in length.
    #[builder(into)]
    #[serde(rename = "intentVersion")]
    pub r#intent_version: String,
}
