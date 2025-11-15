#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KeyRotationPolicy {
    /// An `automatic` block as defined below.
    #[builder(into)]
    #[serde(rename = "automatic")]
    pub r#automatic: Option<Box<super::super::types::keyvault::KeyRotationPolicyAutomatic>>,
    /// Expire a Key Vault Key after given duration as an [ISO 8601 duration](https://en.wikipedia.org/wiki/ISO_8601#Durations).
    #[builder(into)]
    #[serde(rename = "expireAfter")]
    pub r#expire_after: Option<String>,
    /// Notify at a given duration before expiry as an [ISO 8601 duration](https://en.wikipedia.org/wiki/ISO_8601#Durations).
    #[builder(into)]
    #[serde(rename = "notifyBeforeExpiry")]
    pub r#notify_before_expiry: Option<String>,
}
