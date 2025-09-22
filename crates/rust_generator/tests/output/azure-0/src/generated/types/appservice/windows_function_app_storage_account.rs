#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WindowsFunctionAppStorageAccount {
    /// The Access key for the storage account.
    #[builder(into)]
    #[serde(rename = "accessKey")]
    pub r#access_key: String,
    /// The Name of the Storage Account.
    #[builder(into)]
    #[serde(rename = "accountName")]
    pub r#account_name: String,
    /// The path at which to mount the storage share.
    #[builder(into)]
    #[serde(rename = "mountPath")]
    pub r#mount_path: Option<String>,
    /// The name which should be used for this Storage Account.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The Name of the File Share or Container Name for Blob storage.
    #[builder(into)]
    #[serde(rename = "shareName")]
    pub r#share_name: String,
    /// The Azure Storage Type. Possible values include `AzureFiles`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
