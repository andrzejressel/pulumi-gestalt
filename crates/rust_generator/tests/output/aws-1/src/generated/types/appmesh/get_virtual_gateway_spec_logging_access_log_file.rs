#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetVirtualGatewaySpecLoggingAccessLogFile {
    #[builder(into)]
    #[serde(rename = "formats")]
    pub r#formats: Vec<super::super::types::appmesh::GetVirtualGatewaySpecLoggingAccessLogFileFormat>,
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: String,
}
