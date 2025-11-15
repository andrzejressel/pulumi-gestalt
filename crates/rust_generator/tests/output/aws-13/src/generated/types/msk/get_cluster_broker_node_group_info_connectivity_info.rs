#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetClusterBrokerNodeGroupInfoConnectivityInfo {
    #[builder(into)]
    #[serde(rename = "publicAccesses")]
    pub r#public_accesses: Vec<super::super::types::msk::GetClusterBrokerNodeGroupInfoConnectivityInfoPublicAccess>,
    #[builder(into)]
    #[serde(rename = "vpcConnectivities")]
    pub r#vpc_connectivities: Vec<super::super::types::msk::GetClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivity>,
}
