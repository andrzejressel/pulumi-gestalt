#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ReplicationConfigurationDestination {
    /// The availability zone in which the replica should be created. If specified, the replica will be created with One Zone storage. If omitted, regional storage will be used.
    #[builder(into)]
    #[serde(rename = "availabilityZoneName")]
    pub r#availability_zone_name: Option<String>,
    /// The ID of the destination file system for the replication. If no ID is provided, then EFS creates a new file system with the default settings.
    #[builder(into)]
    #[serde(rename = "fileSystemId")]
    pub r#file_system_id: Option<String>,
    /// The Key ID, ARN, alias, or alias ARN of the KMS key that should be used to encrypt the replica file system. If omitted, the default KMS key for EFS `/aws/elasticfilesystem` will be used.
    #[builder(into)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Option<String>,
    /// The region in which the replica should be created.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: Option<String>,
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Option<String>,
}
