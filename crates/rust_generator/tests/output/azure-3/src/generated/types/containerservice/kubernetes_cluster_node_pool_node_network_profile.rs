#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KubernetesClusterNodePoolNodeNetworkProfile {
    /// One or more `allowed_host_ports` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "allowedHostPorts")]
    pub r#allowed_host_ports: Option<Vec<super::super::types::containerservice::KubernetesClusterNodePoolNodeNetworkProfileAllowedHostPort>>,
    /// A list of Application Security Group IDs which should be associated with this Node Pool.
    #[builder(into)]
    #[serde(rename = "applicationSecurityGroupIds")]
    pub r#application_security_group_ids: Option<Vec<String>>,
    /// Specifies a mapping of tags to the instance-level public IPs. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "nodePublicIpTags")]
    pub r#node_public_ip_tags: Option<std::collections::HashMap<String, String>>,
}
