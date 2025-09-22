#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetSpringCloudServiceConfigServerGitSettingRepositoryHttpBasicAuth {
    /// The password used to access the HTTP Basic Authentication Git repository server.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: String,
    /// The username used to access the HTTP Basic Authentication Git repository server.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: String,
}
