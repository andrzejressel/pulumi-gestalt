#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ApplicationApplicationConfigurationApplicationCodeConfiguration {
    /// The location and type of the application code.
    #[builder(into)]
    #[serde(rename = "codeContent")]
    pub r#code_content: Option<Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationApplicationCodeConfigurationCodeContent>>,
    /// Specifies whether the code content is in text or zip format. Valid values: `PLAINTEXT`, `ZIPFILE`.
    #[builder(into)]
    #[serde(rename = "codeContentType")]
    pub r#code_content_type: String,
}
