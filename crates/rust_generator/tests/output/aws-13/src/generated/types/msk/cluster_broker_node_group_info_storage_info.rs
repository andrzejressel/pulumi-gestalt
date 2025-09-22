#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterBrokerNodeGroupInfoStorageInfo {
    /// A block that contains EBS volume information. See below.
    #[builder(into)]
    #[serde(rename = "ebsStorageInfo")]
    pub r#ebs_storage_info: Option<Box<super::super::types::msk::ClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfo>>,
}
