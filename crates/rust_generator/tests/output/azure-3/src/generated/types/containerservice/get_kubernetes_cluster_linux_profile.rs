#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetKubernetesClusterLinuxProfile {
    /// The username associated with the administrator account of the Windows VMs.
    #[builder(into)]
    #[serde(rename = "adminUsername")]
    pub r#admin_username: String,
    /// An `ssh_key` block as defined below.
    #[builder(into)]
    #[serde(rename = "sshKeys")]
    pub r#ssh_keys: Vec<super::super::types::containerservice::GetKubernetesClusterLinuxProfileSshKey>,
}
