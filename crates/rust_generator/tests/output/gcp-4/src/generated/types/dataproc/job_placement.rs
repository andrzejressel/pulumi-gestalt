#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobPlacement {
    /// The name of the cluster where the job will be submitted
    #[builder(into)]
    #[serde(rename = "clusterName")]
    pub r#cluster_name: Box<String>,
    /// Output-only. A cluster UUID generated by the Cloud Dataproc service when the job is submitted
    #[builder(into, default)]
    #[serde(rename = "clusterUuid")]
    pub r#cluster_uuid: Box<Option<String>>,
}
