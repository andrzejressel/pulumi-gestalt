#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CxFlowTransitionRoute {
    /// The condition to evaluate against form parameters or session parameters.
    /// At least one of intent or condition must be specified. When both intent and condition are specified, the transition can only happen when both are fulfilled.
    #[builder(into)]
    #[serde(rename = "condition")]
    pub r#condition: Option<String>,
    /// The unique identifier of an Intent.
    /// Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/intents/<Intent ID>. Indicates that the transition can only happen when the given intent is matched. At least one of intent or condition must be specified. When both intent and condition are specified, the transition can only happen when both are fulfilled.
    #[builder(into)]
    #[serde(rename = "intent")]
    pub r#intent: Option<String>,
    /// (Output)
    /// The unique identifier of this transition route.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The target flow to transition to.
    /// Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>.
    #[builder(into)]
    #[serde(rename = "targetFlow")]
    pub r#target_flow: Option<String>,
    /// The target page to transition to.
    /// Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/flows/<Flow ID>/pages/<Page ID>.
    #[builder(into)]
    #[serde(rename = "targetPage")]
    pub r#target_page: Option<String>,
    /// The fulfillment to call when the condition is satisfied. At least one of triggerFulfillment and target must be specified. When both are defined, triggerFulfillment is executed first.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "triggerFulfillment")]
    pub r#trigger_fulfillment: Box<Option<super::super::types::diagflow::CxFlowTransitionRouteTriggerFulfillment>>,
}
