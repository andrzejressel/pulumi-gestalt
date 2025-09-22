#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetModelsModelSummary {
    /// Customizations that the model supports.
    #[builder(into)]
    #[serde(rename = "customizationsSupporteds")]
    pub r#customizations_supporteds: Vec<String>,
    /// Inference types that the model supports.
    #[builder(into)]
    #[serde(rename = "inferenceTypesSupporteds")]
    pub r#inference_types_supporteds: Vec<String>,
    /// Input modalities that the model supports.
    #[builder(into)]
    #[serde(rename = "inputModalities")]
    pub r#input_modalities: Vec<String>,
    /// Model ARN.
    #[builder(into)]
    #[serde(rename = "modelArn")]
    pub r#model_arn: String,
    /// Model identifier.
    #[builder(into)]
    #[serde(rename = "modelId")]
    pub r#model_id: String,
    /// Model name.
    #[builder(into)]
    #[serde(rename = "modelName")]
    pub r#model_name: String,
    /// Output modalities that the model supports.
    #[builder(into)]
    #[serde(rename = "outputModalities")]
    pub r#output_modalities: Vec<String>,
    /// Model provider name.
    #[builder(into)]
    #[serde(rename = "providerName")]
    pub r#provider_name: String,
    /// Indicates whether the model supports streaming.
    #[builder(into)]
    #[serde(rename = "responseStreamingSupported")]
    pub r#response_streaming_supported: bool,
}
