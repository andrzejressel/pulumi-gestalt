#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VirtualGatewaySpecLoggingAccessLog {
    /// File object to send virtual gateway access logs to.
    #[builder(into)]
    #[serde(rename = "file")]
    pub r#file: Option<Box<super::super::types::appmesh::VirtualGatewaySpecLoggingAccessLogFile>>,
}
