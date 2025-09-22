#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SpringCloudCustomizedAcceleratorGitRepository {
    /// A `basic_auth` block as defined below. Conflicts with `git_repository[0].ssh_auth`. Changing this forces a new Spring Cloud Customized Accelerator to be created.
    #[builder(into)]
    #[serde(rename = "basicAuth")]
    pub r#basic_auth: Option<Box<super::super::types::appplatform::SpringCloudCustomizedAcceleratorGitRepositoryBasicAuth>>,
    /// Specifies the Git repository branch to be used.
    #[builder(into)]
    #[serde(rename = "branch")]
    pub r#branch: Option<String>,
    /// Specifies the ID of the CA Spring Cloud Certificate for https URL of Git repository.
    #[builder(into)]
    #[serde(rename = "caCertificateId")]
    pub r#ca_certificate_id: Option<String>,
    /// Specifies the Git repository commit to be used.
    #[builder(into)]
    #[serde(rename = "commit")]
    pub r#commit: Option<String>,
    /// Specifies the Git repository tag to be used.
    #[builder(into)]
    #[serde(rename = "gitTag")]
    pub r#git_tag: Option<String>,
    /// Specifies the interval for checking for updates to Git or image repository. It should be greater than 10.
    #[builder(into)]
    #[serde(rename = "intervalInSeconds")]
    pub r#interval_in_seconds: Option<i32>,
    /// Specifies the path under the git repository to be treated as the root directory of the accelerator or the fragment (depending on `accelerator_type`).
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Option<String>,
    /// A `ssh_auth` block as defined below. Conflicts with `git_repository[0].basic_auth`. Changing this forces a new Spring Cloud Customized Accelerator to be created.
    #[builder(into)]
    #[serde(rename = "sshAuth")]
    pub r#ssh_auth: Option<Box<super::super::types::appplatform::SpringCloudCustomizedAcceleratorGitRepositorySshAuth>>,
    /// Specifies Git repository URL for the accelerator.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: String,
}
