#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DeliveryPipelineConditionTargetsPresentCondition {
    /// The list of Target names that are missing. For example, projects/{project_id}/locations/{location_name}/targets/{target_name}.
    #[builder(into)]
    #[serde(rename = "missingTargets")]
    pub r#missing_targets: Option<Vec<String>>,
    /// True if there aren't any missing Targets.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Option<bool>,
    /// Output only. Most recent time at which the pipeline was updated.
    #[builder(into)]
    #[serde(rename = "updateTime")]
    pub r#update_time: Option<String>,
}
