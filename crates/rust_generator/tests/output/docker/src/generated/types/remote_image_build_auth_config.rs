#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RemoteImageBuildAuthConfig {
    /// the auth token
    #[builder(into)]
    #[serde(rename = "auth")]
    pub r#auth: Option<String>,
    /// the user emal
    #[builder(into)]
    #[serde(rename = "email")]
    pub r#email: Option<String>,
    /// hostname of the registry
    #[builder(into)]
    #[serde(rename = "hostName")]
    pub r#host_name: String,
    /// the identity token
    #[builder(into)]
    #[serde(rename = "identityToken")]
    pub r#identity_token: Option<String>,
    /// the registry password
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Option<String>,
    /// the registry token
    #[builder(into)]
    #[serde(rename = "registryToken")]
    pub r#registry_token: Option<String>,
    /// the server address
    #[builder(into)]
    #[serde(rename = "serverAddress")]
    pub r#server_address: Option<String>,
    /// the registry user name
    #[builder(into)]
    #[serde(rename = "userName")]
    pub r#user_name: Option<String>,
}
