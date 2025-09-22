#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetWindowsFunctionAppAuthSettingsV2ActiveDirectoryV2 {
    /// The list of allowed Applications for the Default Authorisation Policy.
    #[builder(into)]
    #[serde(rename = "allowedApplications")]
    pub r#allowed_applications: Vec<String>,
    /// The list of Allowed Audiences that are be requested as part of Microsoft Sign-In authentication.
    #[builder(into)]
    #[serde(rename = "allowedAudiences")]
    pub r#allowed_audiences: Vec<String>,
    /// The list of allowed Group Names for the Default Authorisation Policy.
    #[builder(into)]
    #[serde(rename = "allowedGroups")]
    pub r#allowed_groups: Vec<String>,
    /// The list of allowed Identities for the Default Authorisation Policy.
    #[builder(into)]
    #[serde(rename = "allowedIdentities")]
    pub r#allowed_identities: Vec<String>,
    /// The OAuth 2.0 client ID that was created for the app used for authentication.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: String,
    /// The thumbprint of the certificate used for signing purposes.
    #[builder(into)]
    #[serde(rename = "clientSecretCertificateThumbprint")]
    pub r#client_secret_certificate_thumbprint: String,
    /// The app setting name containing the OAuth 2.0 client secret that was created for the app used for authentication.
    #[builder(into)]
    #[serde(rename = "clientSecretSettingName")]
    pub r#client_secret_setting_name: String,
    /// The list of Allowed Client Applications in the JWT Claim.
    #[builder(into)]
    #[serde(rename = "jwtAllowedClientApplications")]
    pub r#jwt_allowed_client_applications: Vec<String>,
    /// The list of Allowed Groups in the JWT Claim.
    #[builder(into)]
    #[serde(rename = "jwtAllowedGroups")]
    pub r#jwt_allowed_groups: Vec<String>,
    /// A map of key-value pairs sent to the Authorisation Endpoint when a user logs in.
    #[builder(into)]
    #[serde(rename = "loginParameters")]
    pub r#login_parameters: std::collections::HashMap<String, String>,
    /// The Azure Tenant Endpoint for the Authenticating Tenant. e.g. `https://login.microsoftonline.com/{tenant-guid}/v2.0/`
    #[builder(into)]
    #[serde(rename = "tenantAuthEndpoint")]
    pub r#tenant_auth_endpoint: String,
    /// Is the www-authenticate provider omitted from the request?
    #[builder(into)]
    #[serde(rename = "wwwAuthenticationDisabled")]
    pub r#www_authentication_disabled: bool,
}
