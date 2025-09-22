#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstanceReadPoolConfig {
    /// Read capacity, i.e. number of nodes in a read pool instance.
    #[builder(into)]
    #[serde(rename = "nodeCount")]
    pub r#node_count: Option<i32>,
}
