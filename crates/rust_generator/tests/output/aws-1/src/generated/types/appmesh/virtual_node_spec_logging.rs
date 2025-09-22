#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VirtualNodeSpecLogging {
    /// Access log configuration for a virtual node.
    #[builder(into)]
    #[serde(rename = "accessLog")]
    pub r#access_log: Box<Option<super::super::types::appmesh::VirtualNodeSpecLoggingAccessLog>>,
}
