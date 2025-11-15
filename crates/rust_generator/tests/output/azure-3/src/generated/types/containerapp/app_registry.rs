#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AppRegistry {
    /// Resource ID for the User Assigned Managed identity to use when pulling from the Container Registry.
    /// 
    /// > **Note:** The Resource ID must be of a User Assigned Managed identity defined in an `identity` block.
    #[builder(into)]
    #[serde(rename = "identity")]
    pub r#identity: Option<String>,
    /// The name of the Secret Reference containing the password value for this user on the Container Registry, `username` must also be supplied.
    #[builder(into)]
    #[serde(rename = "passwordSecretName")]
    pub r#password_secret_name: Option<String>,
    /// The hostname for the Container Registry.
    /// 
    /// The authentication details must also be supplied, `identity` and `username`/`password_secret_name` are mutually exclusive.
    #[builder(into)]
    #[serde(rename = "server")]
    pub r#server: String,
    /// The username to use for this Container Registry, `password_secret_name` must also be supplied..
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Option<String>,
}
