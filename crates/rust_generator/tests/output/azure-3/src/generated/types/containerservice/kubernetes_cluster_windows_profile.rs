#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct KubernetesClusterWindowsProfile {
    /// The Admin Password for Windows VMs. Length must be between 14 and 123 characters.
    #[builder(into)]
    #[serde(rename = "adminPassword")]
    pub r#admin_password: String,
    /// The Admin Username for Windows VMs. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "adminUsername")]
    pub r#admin_username: String,
    /// A `gmsa` block as defined below.
    #[builder(into)]
    #[serde(rename = "gmsa")]
    pub r#gmsa: Option<Box<super::super::types::containerservice::KubernetesClusterWindowsProfileGmsa>>,
    /// Specifies the type of on-premise license which should be used for Node Pool Windows Virtual Machine. At this time the only possible value is `Windows_Server`.
    #[builder(into)]
    #[serde(rename = "license")]
    pub r#license: Option<String>,
}
