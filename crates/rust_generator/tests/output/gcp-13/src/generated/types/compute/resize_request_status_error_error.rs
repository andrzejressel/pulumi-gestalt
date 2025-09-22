#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ResizeRequestStatusErrorError {
    /// (Output)
    /// [Output Only] The error type identifier for this error.
    #[builder(into)]
    #[serde(rename = "code")]
    pub r#code: Option<String>,
    /// (Output)
    /// [Output Only] An optional list of messages that contain the error details. There is a set of defined message types to use for providing details.The syntax depends on the error code. For example, QuotaExceededInfo will have details when the error code is QUOTA_EXCEEDED.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "errorDetails")]
    pub r#error_details: Option<Vec<super::super::types::compute::ResizeRequestStatusErrorErrorErrorDetail>>,
    /// (Output)
    /// Output Only] Indicates the field in the request that caused the error. This property is optional.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Option<String>,
    /// (Output)
    /// The localized error message in the above locale.
    #[builder(into)]
    #[serde(rename = "message")]
    pub r#message: Option<String>,
}
