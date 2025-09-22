#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SpringCloudCustomizedAcceleratorGitRepositoryBasicAuth {
    /// Specifies the password of git repository basic auth.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: String,
    /// Specifies the username of git repository basic auth.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: String,
}
