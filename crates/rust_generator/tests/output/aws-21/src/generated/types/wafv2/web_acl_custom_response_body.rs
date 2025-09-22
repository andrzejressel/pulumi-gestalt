#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WebAclCustomResponseBody {
    /// Payload of the custom response.
    #[builder(into)]
    #[serde(rename = "content")]
    pub r#content: String,
    /// Type of content in the payload that you are defining in the `content` argument. Valid values are `TEXT_PLAIN`, `TEXT_HTML`, or `APPLICATION_JSON`.
    #[builder(into)]
    #[serde(rename = "contentType")]
    pub r#content_type: String,
    /// Unique key identifying the custom response body. This is referenced by the `custom_response_body_key` argument in the `custom_response` block.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: String,
}
