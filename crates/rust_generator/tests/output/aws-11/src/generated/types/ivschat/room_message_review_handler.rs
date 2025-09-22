#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RoomMessageReviewHandler {
    /// The fallback behavior (whether the message
    /// is allowed or denied) if the handler does not return a valid response,
    /// encounters an error, or times out. Valid values: `ALLOW`, `DENY`.
    #[builder(into)]
    #[serde(rename = "fallbackResult")]
    pub r#fallback_result: Option<String>,
    /// ARN of the lambda message review handler function.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: Option<String>,
}
