#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterNodePoolNetworkConfigAdditionalNodeNetworkConfig {
    /// The name or self_link of the Google Compute Engine
    /// network to which the cluster is connected. For Shared VPC, set this to the self link of the
    /// shared network.
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: Option<String>,
    /// The name or self_link of the Google Compute Engine
    /// subnetwork in which the cluster's instances are launched.
    #[builder(into)]
    #[serde(rename = "subnetwork")]
    pub r#subnetwork: Option<String>,
}
