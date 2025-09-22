#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterEncryptionInfo {
    /// You may specify a KMS key short ID or ARN (it will always output an ARN) to use for encrypting your data at rest.  If no key is specified, an AWS managed KMS ('aws/msk' managed service) key will be used for encrypting the data at rest.
    #[builder(into)]
    #[serde(rename = "encryptionAtRestKmsKeyArn")]
    pub r#encryption_at_rest_kms_key_arn: Option<String>,
    /// Configuration block to specify encryption in transit. See below.
    #[builder(into)]
    #[serde(rename = "encryptionInTransit")]
    pub r#encryption_in_transit: Box<Option<super::super::types::msk::ClusterEncryptionInfoEncryptionInTransit>>,
}
