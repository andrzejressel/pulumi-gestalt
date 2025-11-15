#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetPrivateCloudManagementCluster {
    /// The list of the hosts in the management cluster.
    #[builder(into)]
    #[serde(rename = "hosts")]
    pub r#hosts: Vec<String>,
    /// The ID of the management cluster.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: i32,
    /// The size of the management cluster.
    #[builder(into)]
    #[serde(rename = "size")]
    pub r#size: i32,
}
