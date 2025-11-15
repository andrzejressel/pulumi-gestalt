#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegistryTaskRegistryCredentialCustom {
    /// The managed identity assigned to this custom credential. For user assigned identity, the value is the client ID of the identity. For system assigned identity, the value is `[system]`.
    #[builder(into)]
    #[serde(rename = "identity")]
    pub r#identity: Option<String>,
    /// The login server of the custom Container Registry.
    #[builder(into)]
    #[serde(rename = "loginServer")]
    pub r#login_server: String,
    /// The password for logging into the custom Container Registry. It can be either a plain text of password, or a Keyvault Secret ID.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Option<String>,
    /// The username for logging into the custom Container Registry. It can be either a plain text of username, or a Keyvault Secret ID.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Option<String>,
}
