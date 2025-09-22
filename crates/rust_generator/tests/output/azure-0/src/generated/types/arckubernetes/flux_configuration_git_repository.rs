#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FluxConfigurationGitRepository {
    /// Specifies the Base64-encoded HTTPS certificate authority contents used to access git private git repositories over HTTPS.
    #[builder(into)]
    #[serde(rename = "httpsCaCertBase64")]
    pub r#https_ca_cert_base_64: Option<String>,
    /// Specifies the Base64-encoded HTTPS personal access token or password that will be used to access the repository.
    #[builder(into)]
    #[serde(rename = "httpsKeyBase64")]
    pub r#https_key_base_64: Option<String>,
    /// Specifies the plaintext HTTPS username used to access private git repositories over HTTPS.
    #[builder(into)]
    #[serde(rename = "httpsUser")]
    pub r#https_user: Option<String>,
    /// Specifies the name of a local secret on the Kubernetes cluster to use as the authentication secret rather than the managed or user-provided configuration secrets. It must be between 1 and 63 characters. It can contain only lowercase letters, numbers, and hyphens (-). It must start and end with a lowercase letter or number.
    #[builder(into)]
    #[serde(rename = "localAuthReference")]
    pub r#local_auth_reference: Option<String>,
    /// Specifies the source reference type for the GitRepository object. Possible values are `branch`, `commit`, `semver` and `tag`.
    #[builder(into)]
    #[serde(rename = "referenceType")]
    pub r#reference_type: String,
    /// Specifies the source reference value for the GitRepository object.
    #[builder(into)]
    #[serde(rename = "referenceValue")]
    pub r#reference_value: String,
    /// Specifies the Base64-encoded known_hosts value containing public SSH keys required to access private git repositories over SSH.
    #[builder(into)]
    #[serde(rename = "sshKnownHostsBase64")]
    pub r#ssh_known_hosts_base_64: Option<String>,
    /// Specifies the Base64-encoded SSH private key in PEM format.
    #[builder(into)]
    #[serde(rename = "sshPrivateKeyBase64")]
    pub r#ssh_private_key_base_64: Option<String>,
    /// Specifies the interval at which to re-reconcile the cluster git repository source with the remote. Defaults to `600`.
    #[builder(into)]
    #[serde(rename = "syncIntervalInSeconds")]
    pub r#sync_interval_in_seconds: Option<i32>,
    /// Specifies the maximum time to attempt to reconcile the cluster git repository source with the remote. Defaults to `600`.
    #[builder(into)]
    #[serde(rename = "timeoutInSeconds")]
    pub r#timeout_in_seconds: Option<i32>,
    /// Specifies the URL to sync for the flux configuration git repository. It must start with `http://`, `https://`, `git@` or `ssh://`.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: String,
}
