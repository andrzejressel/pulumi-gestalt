#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetConfigurationKeysItem {
    /// The content type of the App Configuration Key.
    #[builder(into)]
    #[serde(rename = "contentType")]
    pub r#content_type: String,
    /// The ETag of the key.
    #[builder(into)]
    #[serde(rename = "etag")]
    pub r#etag: String,
    /// The name of the App Configuration Keys to look up.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: String,
    /// The label of the App Configuration Keys tp look up.
    #[builder(into)]
    #[serde(rename = "label")]
    pub r#label: String,
    /// Is this App Configuration Key be Locked to prevent changes.
    #[builder(into)]
    #[serde(rename = "locked")]
    pub r#locked: bool,
    /// A mapping of tags assigned to the resource.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: std::collections::HashMap<String, String>,
    /// The type of the App Configuration Key. It can either be `kv` (simple [key/value](https://docs.microsoft.com/azure/azure-app-configuration/concept-key-value)) or `vault` (where the value is a reference to a [Key Vault Secret](https://azure.microsoft.com/en-gb/services/key-vault/).
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
    /// The value of the App Configuration Key.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: String,
    /// The ID of the vault secret this App Configuration Key refers to, when `type` is `vault`.
    #[builder(into)]
    #[serde(rename = "vaultKeyReference")]
    pub r#vault_key_reference: String,
}
