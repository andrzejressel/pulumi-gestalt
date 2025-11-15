#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SpringCloudApiPortalSso {
    /// The public identifier for the application.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: Option<String>,
    /// The secret known only to the application and the authorization server.
    #[builder(into)]
    #[serde(rename = "clientSecret")]
    pub r#client_secret: Option<String>,
    /// The URI of Issuer Identifier.
    #[builder(into)]
    #[serde(rename = "issuerUri")]
    pub r#issuer_uri: Option<String>,
    /// It defines the specific actions applications can be allowed to do on a user's behalf.
    #[builder(into)]
    #[serde(rename = "scopes")]
    pub r#scopes: Option<Vec<String>>,
}
