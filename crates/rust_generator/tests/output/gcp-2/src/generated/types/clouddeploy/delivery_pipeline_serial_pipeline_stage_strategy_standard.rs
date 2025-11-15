#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DeliveryPipelineSerialPipelineStageStrategyStandard {
    /// Optional. Configuration for the postdeploy job. If this is not configured, postdeploy job will not be present.
    #[builder(into)]
    #[serde(rename = "postdeploy")]
    pub r#postdeploy: Option<Box<super::super::types::clouddeploy::DeliveryPipelineSerialPipelineStageStrategyStandardPostdeploy>>,
    /// Optional. Configuration for the predeploy job. If this is not configured, predeploy job will not be present.
    #[builder(into)]
    #[serde(rename = "predeploy")]
    pub r#predeploy: Option<Box<super::super::types::clouddeploy::DeliveryPipelineSerialPipelineStageStrategyStandardPredeploy>>,
    /// Whether to verify a deployment.
    #[builder(into)]
    #[serde(rename = "verify")]
    pub r#verify: Option<bool>,
}
