#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SnapshotImportDiskContainer {
    /// The description of the disk image being imported.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// The format of the disk image being imported. One of `VHD` or `VMDK`.
    #[builder(into)]
    #[serde(rename = "format")]
    pub r#format: String,
    /// The URL to the Amazon S3-based disk image being imported. It can either be a https URL (https://..) or an Amazon S3 URL (s3://..). One of `url` or `user_bucket` must be set.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: Option<String>,
    /// The Amazon S3 bucket for the disk image. One of `url` or `user_bucket` must be set. Detailed below.
    #[builder(into)]
    #[serde(rename = "userBucket")]
    pub r#user_bucket: Option<Box<super::super::types::ebs::SnapshotImportDiskContainerUserBucket>>,
}
