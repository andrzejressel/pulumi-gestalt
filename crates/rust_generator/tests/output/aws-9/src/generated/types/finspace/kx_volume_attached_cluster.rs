#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct KxVolumeAttachedCluster {
    #[builder(into)]
    #[serde(rename = "clusterName")]
    pub r#cluster_name: Box<String>,
    #[builder(into)]
    #[serde(rename = "clusterStatus")]
    pub r#cluster_status: Box<String>,
    #[builder(into)]
    #[serde(rename = "clusterType")]
    pub r#cluster_type: Box<String>,
}
