#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetPoolUserAccountLinuxUserConfiguration {
    /// The user ID of the user account.
    #[builder(into)]
    #[serde(rename = "gid")]
    pub r#gid: i32,
    /// The SSH private key for the user account.
    #[builder(into)]
    #[serde(rename = "sshPrivateKey")]
    pub r#ssh_private_key: String,
    /// The group ID for the user account.
    #[builder(into)]
    #[serde(rename = "uid")]
    pub r#uid: i32,
}
