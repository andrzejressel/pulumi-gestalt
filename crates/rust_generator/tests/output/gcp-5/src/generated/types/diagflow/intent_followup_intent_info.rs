#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct IntentFollowupIntentInfo {
    /// The unique identifier of the followup intent.
    /// Format: projects/<Project ID>/agent/intents/<Intent ID>.
    #[builder(into)]
    #[serde(rename = "followupIntentName")]
    pub r#followup_intent_name: Option<String>,
    /// The unique identifier of the parent intent in the chain of followup intents.
    /// Format: projects/<Project ID>/agent/intents/<Intent ID>.
    #[builder(into)]
    #[serde(rename = "parentFollowupIntentName")]
    pub r#parent_followup_intent_name: Option<String>,
}
