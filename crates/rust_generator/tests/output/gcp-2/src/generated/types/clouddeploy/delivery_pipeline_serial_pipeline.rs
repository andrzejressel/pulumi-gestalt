#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DeliveryPipelineSerialPipeline {
    /// Each stage specifies configuration for a `Target`. The ordering of this list defines the promotion flow.
    #[builder(into)]
    #[serde(rename = "stages")]
    pub r#stages: Option<Vec<super::super::types::clouddeploy::DeliveryPipelineSerialPipelineStage>>,
}
