#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DeliveryPipelineSerialPipelineStageStrategyCanaryCanaryDeployment {
    /// Required. The percentage based deployments that will occur as a part of a `Rollout`. List is expected in ascending order and each integer n is 0 <= n < 100.
    #[builder(into)]
    #[serde(rename = "percentages")]
    pub r#percentages: Vec<i32>,
    /// Optional. Configuration for the postdeploy job of the last phase. If this is not configured, postdeploy job will not be present.
    #[builder(into)]
    #[serde(rename = "postdeploy")]
    pub r#postdeploy: Box<Option<super::super::types::clouddeploy::DeliveryPipelineSerialPipelineStageStrategyCanaryCanaryDeploymentPostdeploy>>,
    /// Optional. Configuration for the predeploy job of the first phase. If this is not configured, predeploy job will not be present.
    #[builder(into)]
    #[serde(rename = "predeploy")]
    pub r#predeploy: Box<Option<super::super::types::clouddeploy::DeliveryPipelineSerialPipelineStageStrategyCanaryCanaryDeploymentPredeploy>>,
    /// Whether to run verify tests after each percentage deployment.
    #[builder(into)]
    #[serde(rename = "verify")]
    pub r#verify: Option<bool>,
}
