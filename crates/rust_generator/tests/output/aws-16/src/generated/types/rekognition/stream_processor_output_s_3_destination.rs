#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct StreamProcessorOutputS3Destination {
    /// Name of the Amazon S3 bucket you want to associate with the streaming video project.
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: Option<String>,
    /// The prefix value of the location within the bucket that you want the information to be published to.
    #[builder(into)]
    #[serde(rename = "keyPrefix")]
    pub r#key_prefix: Option<String>,
}
