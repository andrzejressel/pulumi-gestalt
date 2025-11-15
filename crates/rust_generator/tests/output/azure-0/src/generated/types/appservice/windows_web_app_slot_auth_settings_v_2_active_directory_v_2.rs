#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WindowsWebAppSlotAuthSettingsV2ActiveDirectoryV2 {
    /// The list of allowed Applications for the Default Authorisation Policy.
    #[builder(into)]
    #[serde(rename = "allowedApplications")]
    pub r#allowed_applications: Option<Vec<String>>,
    /// Specifies a list of Allowed audience values to consider when validating JWTs issued by Azure Active Directory.
    /// 
    /// > **NOTE:** This is configured on the Authentication Provider side and is Read Only here.
    #[builder(into)]
    #[serde(rename = "allowedAudiences")]
    pub r#allowed_audiences: Option<Vec<String>>,
    /// The list of allowed Group Names for the Default Authorisation Policy.
    #[builder(into)]
    #[serde(rename = "allowedGroups")]
    pub r#allowed_groups: Option<Vec<String>>,
    /// The list of allowed Identities for the Default Authorisation Policy.
    #[builder(into)]
    #[serde(rename = "allowedIdentities")]
    pub r#allowed_identities: Option<Vec<String>>,
    /// The ID of the Client to use to authenticate with Azure Active Directory.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: String,
    /// The thumbprint of the certificate used for signing purposes.
    #[builder(into)]
    #[serde(rename = "clientSecretCertificateThumbprint")]
    pub r#client_secret_certificate_thumbprint: Option<String>,
    /// The App Setting name that contains the client secret of the Client.
    /// 
    /// !> **NOTE:** A setting with this name must exist in `app_settings` to function correctly.
    #[builder(into)]
    #[serde(rename = "clientSecretSettingName")]
    pub r#client_secret_setting_name: Option<String>,
    /// A list of Allowed Client Applications in the JWT Claim.
    #[builder(into)]
    #[serde(rename = "jwtAllowedClientApplications")]
    pub r#jwt_allowed_client_applications: Option<Vec<String>>,
    /// A list of Allowed Groups in the JWT Claim.
    #[builder(into)]
    #[serde(rename = "jwtAllowedGroups")]
    pub r#jwt_allowed_groups: Option<Vec<String>>,
    /// A map of key-value pairs to send to the Authorisation Endpoint when a user logs in.
    #[builder(into)]
    #[serde(rename = "loginParameters")]
    pub r#login_parameters: Option<std::collections::HashMap<String, String>>,
    /// The Azure Tenant Endpoint for the Authenticating Tenant. e.g. `https://login.microsoftonline.com/{tenant-guid}/v2.0/`
    /// 
    /// > **NOTE:** [Here](https://learn.microsoft.com/en-us/entra/identity-platform/authentication-national-cloud#microsoft-entra-authentication-endpoints) is a list of possible authentication endpoints based on the cloud environment. [Here](https://learn.microsoft.com/en-us/azure/app-service/configure-authentication-provider-aad?tabs=workforce-tenant) is more information to better understand how to configure authentication for Azure App Service or Azure Functions.
    #[builder(into)]
    #[serde(rename = "tenantAuthEndpoint")]
    pub r#tenant_auth_endpoint: String,
    /// Should the www-authenticate provider should be omitted from the request? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "wwwAuthenticationDisabled")]
    pub r#www_authentication_disabled: Option<bool>,
}
