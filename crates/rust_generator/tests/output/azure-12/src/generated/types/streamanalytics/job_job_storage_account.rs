#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobJobStorageAccount {
    /// The account key for the Azure storage account.
    #[builder(into)]
    #[serde(rename = "accountKey")]
    pub r#account_key: String,
    /// The name of the Azure storage account.
    #[builder(into)]
    #[serde(rename = "accountName")]
    pub r#account_name: String,
    /// The authentication mode of the storage account. The only supported value is `ConnectionString`. Defaults to `ConnectionString`.
    #[builder(into)]
    #[serde(rename = "authenticationMode")]
    pub r#authentication_mode: Option<String>,
}
