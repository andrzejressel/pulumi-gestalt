#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AppConnectionAuthentication {
    /// Service principal certificate for `servicePrincipal` auth. Should be specified when `type` is set to `servicePrincipalCertificate`.
    #[builder(into)]
    #[serde(rename = "certificate")]
    pub r#certificate: Option<String>,
    /// Client ID for `userAssignedIdentity` or `servicePrincipal` auth. Should be specified when `type` is set to `servicePrincipalSecret` or `servicePrincipalCertificate`. When `type` is set to `userAssignedIdentity`, `client_id` and `subscription_id` should be either both specified or both not specified.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: Option<String>,
    /// Username or account name for secret auth. `name` and `secret` should be either both specified or both not specified when `type` is set to `secret`.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Principal ID for `servicePrincipal` auth. Should be specified when `type` is set to `servicePrincipalSecret` or `servicePrincipalCertificate`.
    #[builder(into)]
    #[serde(rename = "principalId")]
    pub r#principal_id: Option<String>,
    /// Password or account key for secret auth. `secret` and `name` should be either both specified or both not specified when `type` is set to `secret`.
    #[builder(into)]
    #[serde(rename = "secret")]
    pub r#secret: Option<String>,
    /// Subscription ID for `userAssignedIdentity`. `subscription_id` and `client_id` should be either both specified or both not specified.
    #[builder(into)]
    #[serde(rename = "subscriptionId")]
    pub r#subscription_id: Option<String>,
    /// The authentication type. Possible values are `systemAssignedIdentity`, `userAssignedIdentity`, `servicePrincipalSecret`, `servicePrincipalCertificate`, `secret`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
