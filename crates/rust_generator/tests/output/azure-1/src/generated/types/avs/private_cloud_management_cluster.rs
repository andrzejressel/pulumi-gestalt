#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PrivateCloudManagementCluster {
    /// A list of hosts in the management cluster.
    #[builder(into)]
    #[serde(rename = "hosts")]
    pub r#hosts: Option<Vec<String>>,
    /// The ID of the management cluster.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<i32>,
    /// The size of the management cluster. This field can not updated with `internet_connection_enabled` together.
    #[builder(into)]
    #[serde(rename = "size")]
    pub r#size: i32,
}
