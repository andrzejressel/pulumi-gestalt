#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkspaceServerlessCompute {
    /// Should serverless compute nodes deployed in a custom Virtual Network have public IP addresses enabled for a workspace with private endpoint? Defaults to `false`.
    /// 
    /// > **Note:** `public_ip_enabled` cannot be updated from `true` to `false` when `subnet_id` is not set. `public_ip_enabled` must be set to `true` if `subnet_id` is not set and when `public_network_access_enabled` is `false`.
    #[builder(into)]
    #[serde(rename = "publicIpEnabled")]
    pub r#public_ip_enabled: Option<bool>,
    /// The ID of an existing Virtual Network Subnet in which the serverless compute nodes should be deployed to.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Option<String>,
}
