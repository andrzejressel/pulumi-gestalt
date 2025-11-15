#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DeliveryPipelineCondition {
    /// Details around the Pipeline's overall status.
    #[builder(into)]
    #[serde(rename = "pipelineReadyConditions")]
    pub r#pipeline_ready_conditions: Option<Vec<super::super::types::clouddeploy::DeliveryPipelineConditionPipelineReadyCondition>>,
    /// Details around targets enumerated in the pipeline.
    #[builder(into)]
    #[serde(rename = "targetsPresentConditions")]
    pub r#targets_present_conditions: Option<Vec<super::super::types::clouddeploy::DeliveryPipelineConditionTargetsPresentCondition>>,
    /// Details on the whether the targets enumerated in the pipeline are of the same type.
    #[builder(into)]
    #[serde(rename = "targetsTypeConditions")]
    pub r#targets_type_conditions: Option<Vec<super::super::types::clouddeploy::DeliveryPipelineConditionTargetsTypeCondition>>,
}
