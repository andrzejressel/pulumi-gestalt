#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApiDiagnosticFrontendResponse {
    /// Number of payload bytes to log (up to 8192).
    #[builder(into)]
    #[serde(rename = "bodyBytes")]
    pub r#body_bytes: Option<i32>,
    /// A `data_masking` block as defined below.
    #[builder(into)]
    #[serde(rename = "dataMasking")]
    pub r#data_masking: Option<Box<super::super::types::apimanagement::ApiDiagnosticFrontendResponseDataMasking>>,
    /// Specifies a list of headers to log.
    #[builder(into)]
    #[serde(rename = "headersToLogs")]
    pub r#headers_to_logs: Option<Vec<String>>,
}
