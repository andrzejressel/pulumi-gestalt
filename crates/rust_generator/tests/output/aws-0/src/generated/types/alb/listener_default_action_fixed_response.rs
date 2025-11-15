#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ListenerDefaultActionFixedResponse {
    /// Content type. Valid values are `text/plain`, `text/css`, `text/html`, `application/javascript` and `application/json`.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "contentType")]
    pub r#content_type: String,
    /// Message body.
    #[builder(into)]
    #[serde(rename = "messageBody")]
    pub r#message_body: Option<String>,
    /// HTTP response code. Valid values are `2XX`, `4XX`, or `5XX`.
    #[builder(into)]
    #[serde(rename = "statusCode")]
    pub r#status_code: Option<String>,
}
