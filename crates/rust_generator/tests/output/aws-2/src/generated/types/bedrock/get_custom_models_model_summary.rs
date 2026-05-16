#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetCustomModelsModelSummary {
    /// Creation time of the model.
    #[builder(into)]
    #[serde(rename = "creationTime")]
    pub r#creation_time: String,
    /// The ARN of the custom model.
    #[builder(into)]
    #[serde(rename = "modelArn")]
    pub r#model_arn: String,
    /// The name of the custom model.
    #[builder(into)]
    #[serde(rename = "modelName")]
    pub r#model_name: String,
}
