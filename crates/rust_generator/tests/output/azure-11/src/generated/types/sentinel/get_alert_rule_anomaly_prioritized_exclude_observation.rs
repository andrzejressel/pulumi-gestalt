#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetAlertRuleAnomalyPrioritizedExcludeObservation {
    /// The description of the threshold observation.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: String,
    /// The excluded value per `description`.
    #[builder(into)]
    #[serde(rename = "exclude")]
    pub r#exclude: String,
    /// The guid of this Sentinel Alert Rule Template. Either `display_name` or `name` have to be specified.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The prioritized value per `description`.
    #[builder(into)]
    #[serde(rename = "prioritize")]
    pub r#prioritize: String,
}
