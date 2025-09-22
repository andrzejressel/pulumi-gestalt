#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AnalysisSourceEntitySourceTemplateDataSetReference {
    /// Dataset Amazon Resource Name (ARN).
    #[builder(into)]
    #[serde(rename = "dataSetArn")]
    pub r#data_set_arn: String,
    /// Dataset placeholder.
    #[builder(into)]
    #[serde(rename = "dataSetPlaceholder")]
    pub r#data_set_placeholder: String,
}
