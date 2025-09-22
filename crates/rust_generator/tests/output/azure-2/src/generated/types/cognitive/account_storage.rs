#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AccountStorage {
    /// The client ID of the managed identity associated with the storage resource.
    /// 
    /// > **NOTE:** Not all `kind` support a `storage` block. For example the `kind` `OpenAI` does not support it.
    #[builder(into)]
    #[serde(rename = "identityClientId")]
    pub r#identity_client_id: Option<String>,
    /// Full resource id of a Microsoft.Storage resource.
    #[builder(into)]
    #[serde(rename = "storageAccountId")]
    pub r#storage_account_id: String,
}
