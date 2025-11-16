#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfo {
    #[builder(into)]
    #[serde(rename = "provisionedThroughputs")]
    pub r#provisioned_throughputs: Vec<super::super::types::msk::GetClusterBrokerNodeGroupInfoStorageInfoEbsStorageInfoProvisionedThroughput>,
    #[builder(into)]
    #[serde(rename = "volumeSize")]
    pub r#volume_size: i32,
}
