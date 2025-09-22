#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SpringCloudDevToolPortalSso {
    /// Specifies the public identifier for the application.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: Option<String>,
    /// Specifies the secret known only to the application and the authorization server.
    #[builder(into)]
    #[serde(rename = "clientSecret")]
    pub r#client_secret: Option<String>,
    /// Specifies the URI of a JSON file with generic OIDC provider configuration.
    #[builder(into)]
    #[serde(rename = "metadataUrl")]
    pub r#metadata_url: Option<String>,
    /// Specifies a list of specific actions applications can be allowed to do on a user's behalf.
    #[builder(into)]
    #[serde(rename = "scopes")]
    pub r#scopes: Option<Vec<String>>,
}
