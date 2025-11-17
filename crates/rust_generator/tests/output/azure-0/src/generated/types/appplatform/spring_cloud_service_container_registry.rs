#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SpringCloudServiceContainerRegistry {
    /// Specifies the name of the container registry.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Specifies the password of the container registry.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: String,
    /// Specifies the login server of the container registry.
    #[builder(into)]
    #[serde(rename = "server")]
    pub r#server: String,
    /// Specifies the username of the container registry.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: String,
}
