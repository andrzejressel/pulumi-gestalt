#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PoolUserAccountLinuxUserConfiguration {
    /// The user ID of the user account. The `uid` and `gid` properties must be specified together or not at all. If not specified the underlying operating system picks the uid.
    #[builder(into)]
    #[serde(rename = "gid")]
    pub r#gid: Option<i32>,
    /// The SSH private key for the user account. The private key must not be password protected. The private key is used to automatically configure asymmetric-key based authentication for SSH between nodes in a Linux pool when the pool's enableInterNodeCommunication property is true (it is ignored if enableInterNodeCommunication is false). It does this by placing the key pair into the user's .ssh directory. If not specified, password-less SSH is not configured between nodes (no modification of the user's .ssh directory is done).
    #[builder(into)]
    #[serde(rename = "sshPrivateKey")]
    pub r#ssh_private_key: Option<String>,
    /// The group ID for the user account. The `uid` and `gid` properties must be specified together or not at all. If not specified the underlying operating system picks the gid.
    #[builder(into)]
    #[serde(rename = "uid")]
    pub r#uid: Option<i32>,
}
