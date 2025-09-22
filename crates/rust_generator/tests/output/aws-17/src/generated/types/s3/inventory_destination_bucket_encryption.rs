#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InventoryDestinationBucketEncryption {
    /// Specifies to use server-side encryption with AWS KMS-managed keys to encrypt the inventory file (documented below).
    #[builder(into)]
    #[serde(rename = "sseKms")]
    pub r#sse_kms: Option<Box<super::super::types::s3::InventoryDestinationBucketEncryptionSseKms>>,
    /// Specifies to use server-side encryption with Amazon S3-managed keys (SSE-S3) to encrypt the inventory file.
    #[builder(into)]
    #[serde(rename = "sseS3")]
    pub r#sse_s_3: Option<Box<super::super::types::s3::InventoryDestinationBucketEncryptionSseS3>>,
}
