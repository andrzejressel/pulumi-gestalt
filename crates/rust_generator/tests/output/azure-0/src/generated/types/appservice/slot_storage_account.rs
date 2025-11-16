#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SlotStorageAccount {
    /// The access key for the storage account.
    #[builder(into)]
    #[serde(rename = "accessKey")]
    pub r#access_key: String,
    /// The name of the storage account.
    #[builder(into)]
    #[serde(rename = "accountName")]
    pub r#account_name: String,
    /// The path to mount the storage within the site's runtime environment.
    #[builder(into)]
    #[serde(rename = "mountPath")]
    pub r#mount_path: Option<String>,
    /// The name of the storage account identifier.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The name of the file share (container name, for Blob storage).
    #[builder(into)]
    #[serde(rename = "shareName")]
    pub r#share_name: String,
    /// The type of storage. Possible values are `AzureBlob` and `AzureFiles`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
