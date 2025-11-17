#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SecretReplicationUserManagedReplica {
    /// Customer Managed Encryption for the secret.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "customerManagedEncryption")]
    pub r#customer_managed_encryption: Option<Box<super::super::types::secretmanager::SecretReplicationUserManagedReplicaCustomerManagedEncryption>>,
    /// The canonical IDs of the location to replicate data. For example: "us-east1".
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: String,
}
