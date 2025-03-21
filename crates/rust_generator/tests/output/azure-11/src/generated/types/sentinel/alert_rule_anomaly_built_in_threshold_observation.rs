#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AlertRuleAnomalyBuiltInThresholdObservation {
    /// The description of the threshold observation.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// The max value of the threshold observation.
    #[builder(into, default)]
    #[serde(rename = "max")]
    pub r#max: Box<Option<String>>,
    /// The min value of the threshold observation.
    #[builder(into, default)]
    #[serde(rename = "min")]
    pub r#min: Box<Option<String>>,
    /// The Name of the built-in Anomaly Alert Rule.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// The value of the threshold observation.
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}
