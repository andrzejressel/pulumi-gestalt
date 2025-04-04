#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VirtualGatewaySpecLoggingAccessLogFile {
    /// The specified format for the logs.
    #[builder(into, default)]
    #[serde(rename = "format")]
    pub r#format: Box<Option<super::super::types::appmesh::VirtualGatewaySpecLoggingAccessLogFileFormat>>,
    /// File path to write access logs to. You can use `/dev/stdout` to send access logs to standard out. Must be between 1 and 255 characters in length.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
}
