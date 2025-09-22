#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CatalogCatalogAdogit {
    #[builder(into)]
    #[serde(rename = "branch")]
    pub r#branch: String,
    #[builder(into)]
    #[serde(rename = "keyVaultKeyUrl")]
    pub r#key_vault_key_url: String,
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: String,
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: String,
}
