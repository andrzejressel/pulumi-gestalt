#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterNodeTypeApplicationPorts {
    /// The end of the Application Port Range on this Node Type.
    #[builder(into)]
    #[serde(rename = "endPort")]
    pub r#end_port: Box<i32>,
    /// The start of the Application Port Range on this Node Type.
    #[builder(into)]
    #[serde(rename = "startPort")]
    pub r#start_port: Box<i32>,
}
