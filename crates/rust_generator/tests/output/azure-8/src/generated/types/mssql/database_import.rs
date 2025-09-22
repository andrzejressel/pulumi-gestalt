#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DatabaseImport {
    /// Specifies the name of the SQL administrator.
    #[builder(into)]
    #[serde(rename = "administratorLogin")]
    pub r#administrator_login: String,
    /// Specifies the password of the SQL administrator.
    #[builder(into)]
    #[serde(rename = "administratorLoginPassword")]
    pub r#administrator_login_password: String,
    /// Specifies the type of authentication used to access the server. Valid values are `SQL` or `ADPassword`.
    #[builder(into)]
    #[serde(rename = "authenticationType")]
    pub r#authentication_type: String,
    /// The resource id for the storage account used to store BACPAC file. If set, private endpoint connection will be created for the storage account. Must match storage account used for storage_uri parameter.
    #[builder(into)]
    #[serde(rename = "storageAccountId")]
    pub r#storage_account_id: Option<String>,
    /// Specifies the access key for the storage account.
    #[builder(into)]
    #[serde(rename = "storageKey")]
    pub r#storage_key: String,
    /// Specifies the type of access key for the storage account. Valid values are `StorageAccessKey` or `SharedAccessKey`.
    #[builder(into)]
    #[serde(rename = "storageKeyType")]
    pub r#storage_key_type: String,
    /// Specifies the blob URI of the .bacpac file.
    #[builder(into)]
    #[serde(rename = "storageUri")]
    pub r#storage_uri: String,
}
