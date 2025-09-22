#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AccountKeyVaultReference {
    /// The Azure identifier of the Azure KeyVault to use.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// The HTTPS URL of the Azure KeyVault to use.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: String,
}
