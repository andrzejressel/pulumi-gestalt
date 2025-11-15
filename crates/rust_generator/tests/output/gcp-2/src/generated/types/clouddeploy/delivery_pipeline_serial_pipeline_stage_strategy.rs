#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DeliveryPipelineSerialPipelineStageStrategy {
    /// Canary deployment strategy provides progressive percentage based deployments to a Target.
    #[builder(into)]
    #[serde(rename = "canary")]
    pub r#canary: Option<Box<super::super::types::clouddeploy::DeliveryPipelineSerialPipelineStageStrategyCanary>>,
    /// Standard deployment strategy executes a single deploy and allows verifying the deployment.
    #[builder(into)]
    #[serde(rename = "standard")]
    pub r#standard: Option<Box<super::super::types::clouddeploy::DeliveryPipelineSerialPipelineStageStrategyStandard>>,
}
