#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AlertRuleAnomalyDuplicateThresholdObservation {
    /// The description of the threshold observation.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// The max value of the threshold observation.
    #[builder(into)]
    #[serde(rename = "max")]
    pub r#max: Option<String>,
    /// The min value of the threshold observation.
    #[builder(into)]
    #[serde(rename = "min")]
    pub r#min: Option<String>,
    /// The name of the threshold observation.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The value of the threshold observation.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: String,
}
