#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterConfigurationInfo {
    /// Amazon Resource Name (ARN) of the MSK Configuration to use in the cluster.
    #[builder(into)]
    #[serde(rename = "arn")]
    pub r#arn: String,
    /// Revision of the MSK Configuration to use in the cluster.
    #[builder(into)]
    #[serde(rename = "revision")]
    pub r#revision: i32,
}
