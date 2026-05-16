#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
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
