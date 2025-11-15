#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CxPageTransitionRouteTriggerFulfillmentSetParameterAction {
    /// Display name of the parameter.
    #[builder(into)]
    #[serde(rename = "parameter")]
    pub r#parameter: Option<String>,
    /// The new JSON-encoded value of the parameter. A null value clears the parameter.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}
