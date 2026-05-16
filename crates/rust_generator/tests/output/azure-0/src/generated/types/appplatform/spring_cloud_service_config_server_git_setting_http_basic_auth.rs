#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SpringCloudServiceConfigServerGitSettingHttpBasicAuth {
    /// The password used to access the Git repository server, required when the Git repository server supports HTTP Basic Authentication.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: String,
    /// The username that's used to access the Git repository server, required when the Git repository server supports HTTP Basic Authentication.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: String,
}
