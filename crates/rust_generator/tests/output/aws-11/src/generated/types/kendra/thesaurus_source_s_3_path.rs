#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ThesaurusSourceS3Path {
    /// The name of the S3 bucket that contains the file.
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: String,
    /// The name of the file.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: String,
}
