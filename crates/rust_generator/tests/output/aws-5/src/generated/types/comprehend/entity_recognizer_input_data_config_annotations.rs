#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EntityRecognizerInputDataConfigAnnotations {
    /// Location of training annotations.
    #[builder(into)]
    #[serde(rename = "s3Uri")]
    pub r#s_3_uri: String,
    #[builder(into)]
    #[serde(rename = "testS3Uri")]
    pub r#test_s_3_uri: Option<String>,
}
