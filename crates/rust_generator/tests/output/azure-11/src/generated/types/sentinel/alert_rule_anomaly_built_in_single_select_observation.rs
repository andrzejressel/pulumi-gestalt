#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AlertRuleAnomalyBuiltInSingleSelectObservation {
    /// The description of the threshold observation.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// The Name of the built-in Anomaly Alert Rule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// A list of supported values of the single select observation.
    #[builder(into)]
    #[serde(rename = "supportedValues")]
    pub r#supported_values: Option<Vec<String>>,
    /// The value of the threshold observation.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}
