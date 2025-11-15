#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct V2QueuedResourceTpuNodeSpecNode {
    /// TPU accelerator type for the TPU. If not specified, this defaults to 'v2-8'.
    #[builder(into)]
    #[serde(rename = "acceleratorType")]
    pub r#accelerator_type: Option<String>,
    /// Text description of the TPU.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Runtime version for the TPU.
    #[builder(into)]
    #[serde(rename = "runtimeVersion")]
    pub r#runtime_version: String,
}
