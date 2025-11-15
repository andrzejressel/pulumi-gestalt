#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetSecretsSecret {
    /// Custom metadata about the secret.
    #[builder(into)]
    #[serde(rename = "annotations")]
    pub r#annotations: std::collections::HashMap<String, String>,
    /// The time at which the Secret was created.
    #[builder(into)]
    #[serde(rename = "createTime")]
    pub r#create_time: String,
    #[builder(into)]
    #[serde(rename = "effectiveAnnotations")]
    pub r#effective_annotations: std::collections::HashMap<String, String>,
    #[builder(into)]
    #[serde(rename = "effectiveLabels")]
    pub r#effective_labels: std::collections::HashMap<String, String>,
    /// Timestamp in UTC when the Secret is scheduled to expire.
    #[builder(into)]
    #[serde(rename = "expireTime")]
    pub r#expire_time: String,
    /// The labels assigned to this Secret.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: std::collections::HashMap<String, String>,
    /// The resource name of the Pub/Sub topic that will be published to.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The ID of the project.
    #[builder(into)]
    #[serde(rename = "project")]
    pub r#project: String,
    /// The combination of labels configured directly on the resource
    ///  and default labels configured on the provider.
    #[builder(into)]
    #[serde(rename = "pulumiLabels")]
    pub r#pulumi_labels: std::collections::HashMap<String, String>,
    /// The replication policy of the secret data attached to the Secret.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "replications")]
    pub r#replications: Vec<super::super::types::secretmanager::GetSecretsSecretReplication>,
    /// The rotation time and period for a Secret.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "rotations")]
    pub r#rotations: Vec<super::super::types::secretmanager::GetSecretsSecretRotation>,
    /// This must be unique within the project.
    #[builder(into)]
    #[serde(rename = "secretId")]
    pub r#secret_id: String,
    /// A list of up to 10 Pub/Sub topics to which messages are published when control plane operations are called on the secret or its versions.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "topics")]
    pub r#topics: Vec<super::super::types::secretmanager::GetSecretsSecretTopic>,
    /// The TTL for the Secret.
    /// A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s".
    /// Only one of 'ttl' or 'expire_time' can be provided.
    #[builder(into)]
    #[serde(rename = "ttl")]
    pub r#ttl: String,
    /// Mapping from version alias to version name.
    #[builder(into)]
    #[serde(rename = "versionAliases")]
    pub r#version_aliases: std::collections::HashMap<String, String>,
    /// The version destroy ttl for the secret version.
    #[builder(into)]
    #[serde(rename = "versionDestroyTtl")]
    pub r#version_destroy_ttl: String,
}
