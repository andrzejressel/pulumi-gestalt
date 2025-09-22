#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstanceNetworkConfig {
    /// Optional. Type of connection for establishing private IP connectivity between the Data Fusion customer project VPC and
    /// the corresponding tenant project from a predefined list of available connection modes.
    /// If this field is unspecified for a private instance, VPC peering is used.
    /// Possible values are: `VPC_PEERING`, `PRIVATE_SERVICE_CONNECT_INTERFACES`.
    #[builder(into)]
    #[serde(rename = "connectionType")]
    pub r#connection_type: Option<String>,
    /// The IP range in CIDR notation to use for the managed Data Fusion instance
    /// nodes. This range must not overlap with any other ranges used in the Data Fusion instance network.
    #[builder(into)]
    #[serde(rename = "ipAllocation")]
    pub r#ip_allocation: Option<String>,
    /// Name of the network in the project with which the tenant project
    /// will be peered for executing pipelines. In case of shared VPC where the network resides in another host
    /// project the network should specified in the form of projects/{host-project-id}/global/networks/{network}
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: Option<String>,
    /// Optional. Configuration for Private Service Connect.
    /// This is required only when using connection type PRIVATE_SERVICE_CONNECT_INTERFACES.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "privateServiceConnectConfig")]
    pub r#private_service_connect_config: Box<Option<super::super::types::datafusion::InstanceNetworkConfigPrivateServiceConnectConfig>>,
}
