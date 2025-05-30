#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetSecretsSecretReplicationUserManaged {
    /// The list of Replicas for this Secret.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "replicas")]
    pub r#replicas: Box<Vec<super::super::types::secretmanager::GetSecretsSecretReplicationUserManagedReplica>>,
}
