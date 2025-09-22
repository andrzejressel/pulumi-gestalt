#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FlowDefinitionHumanLoopActivationConfig {
    /// defines under what conditions SageMaker creates a human loop. See Human Loop Activation Conditions Config details below.
    #[builder(into)]
    #[serde(rename = "humanLoopActivationConditionsConfig")]
    pub r#human_loop_activation_conditions_config: Option<Box<super::super::types::sagemaker::FlowDefinitionHumanLoopActivationConfigHumanLoopActivationConditionsConfig>>,
}
