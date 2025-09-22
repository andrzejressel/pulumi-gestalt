#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceTaskSpecContainerSpecPrivileges {
    /// CredentialSpec for managed service account (Windows only)
    #[builder(into)]
    #[serde(rename = "credentialSpec")]
    pub r#credential_spec: Option<Box<super::types::ServiceTaskSpecContainerSpecPrivilegesCredentialSpec>>,
    /// SELinux labels of the container
    #[builder(into)]
    #[serde(rename = "seLinuxContext")]
    pub r#se_linux_context: Option<Box<super::types::ServiceTaskSpecContainerSpecPrivilegesSeLinuxContext>>,
}
