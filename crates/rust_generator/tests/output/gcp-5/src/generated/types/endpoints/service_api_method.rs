#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceApiMethod {
    /// The simple name of the endpoint as described in the config.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The type URL for the request to this API.
    #[builder(into)]
    #[serde(rename = "requestType")]
    pub r#request_type: Option<String>,
    /// The type URL for the response from this API.
    #[builder(into)]
    #[serde(rename = "responseType")]
    pub r#response_type: Option<String>,
    /// `SYNTAX_PROTO2` or `SYNTAX_PROTO3`.
    #[builder(into)]
    #[serde(rename = "syntax")]
    pub r#syntax: Option<String>,
}
