#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FunctionAppAuthSettingsActiveDirectory {
    /// Allowed audience values to consider when validating JWTs issued by Azure Active Directory.
    #[builder(into)]
    #[serde(rename = "allowedAudiences")]
    pub r#allowed_audiences: Option<Vec<String>>,
    /// The Client ID of this relying party application. Enables OpenIDConnection authentication with Azure Active Directory.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: String,
    /// The Client Secret of this relying party application. If no secret is provided, implicit flow will be used.
    #[builder(into)]
    #[serde(rename = "clientSecret")]
    pub r#client_secret: Option<String>,
}
