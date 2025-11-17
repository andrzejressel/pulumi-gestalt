#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SecretReplica {
    /// ARN, Key ID, or Alias of the AWS KMS key within the region secret is replicated to. If one is not specified, then Secrets Manager defaults to using the AWS account's default KMS key (`aws/secretsmanager`) in the region or creates one for use if non-existent.
    #[builder(into)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Option<String>,
    /// Date that you last accessed the secret in the Region.
    #[builder(into)]
    #[serde(rename = "lastAccessedDate")]
    pub r#last_accessed_date: Option<String>,
    /// Region for replicating the secret.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: String,
    /// Status can be `InProgress`, `Failed`, or `InSync`.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Option<String>,
    /// Message such as `Replication succeeded` or `Secret with this name already exists in this region`.
    #[builder(into)]
    #[serde(rename = "statusMessage")]
    pub r#status_message: Option<String>,
}
