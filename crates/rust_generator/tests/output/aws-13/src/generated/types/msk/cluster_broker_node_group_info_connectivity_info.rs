#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterBrokerNodeGroupInfoConnectivityInfo {
    /// Access control settings for brokers. See below.
    #[builder(into)]
    #[serde(rename = "publicAccess")]
    pub r#public_access: Option<Box<super::super::types::msk::ClusterBrokerNodeGroupInfoConnectivityInfoPublicAccess>>,
    /// VPC connectivity access control for brokers. See below.
    #[builder(into)]
    #[serde(rename = "vpcConnectivity")]
    pub r#vpc_connectivity: Option<Box<super::super::types::msk::ClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivity>>,
}
