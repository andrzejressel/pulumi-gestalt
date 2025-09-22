#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PipelineContentConfig {
    /// The Amazon S3 bucket in which you want Elastic Transcoder to save transcoded files and playlists.
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: Option<String>,
    /// The Amazon S3 storage class, `Standard` or `ReducedRedundancy`, that you want Elastic Transcoder to assign to the files and playlists that it stores in your Amazon S3 bucket.
    #[builder(into)]
    #[serde(rename = "storageClass")]
    pub r#storage_class: Option<String>,
}
