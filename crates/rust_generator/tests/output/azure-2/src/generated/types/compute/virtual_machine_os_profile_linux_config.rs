#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualMachineOsProfileLinuxConfig {
    /// Specifies whether password authentication should be disabled. If set to `false`, an `admin_password` must be specified.
    #[builder(into)]
    #[serde(rename = "disablePasswordAuthentication")]
    pub r#disable_password_authentication: bool,
    /// One or more `ssh_keys` blocks as defined below. This field is required if `disable_password_authentication` is set to `true`.
    #[builder(into)]
    #[serde(rename = "sshKeys")]
    pub r#ssh_keys: Option<Vec<super::super::types::compute::VirtualMachineOsProfileLinuxConfigSshKey>>,
}
