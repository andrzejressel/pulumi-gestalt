#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SettingsAccessSettingsWorkforceIdentitySettingsOauth2 {
    /// The OAuth 2.0 client ID registered in the workforce identity
    /// federation OAuth 2.0 Server.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: Option<String>,
    /// Input only. The OAuth 2.0 client secret created while registering
    /// the client ID.
    #[builder(into)]
    #[serde(rename = "clientSecret")]
    pub r#client_secret: Option<String>,
    /// Output only. SHA256 hash value for the client secret. This field
    /// is returned by IAP when the settings are retrieved.
    #[builder(into)]
    #[serde(rename = "clientSecretSha256")]
    pub r#client_secret_sha_256: Option<String>,
}
