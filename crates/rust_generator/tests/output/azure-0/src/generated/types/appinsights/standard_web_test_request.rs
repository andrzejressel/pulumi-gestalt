#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StandardWebTestRequest {
    /// The WebTest request body.
    #[builder(into)]
    #[serde(rename = "body")]
    pub r#body: Option<String>,
    /// Should the following of redirects be enabled? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "followRedirectsEnabled")]
    pub r#follow_redirects_enabled: Option<bool>,
    /// One or more `header` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "headers")]
    pub r#headers: Option<Vec<super::super::types::appinsights::StandardWebTestRequestHeader>>,
    /// Which HTTP verb to use for the call. Options are 'GET', 'POST', 'PUT', 'PATCH', 'DELETE', 'HEAD', and 'OPTIONS'. Defaults to `GET`.
    #[builder(into)]
    #[serde(rename = "httpVerb")]
    pub r#http_verb: Option<String>,
    /// Should the parsing of dependend requests be enabled? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "parseDependentRequestsEnabled")]
    pub r#parse_dependent_requests_enabled: Option<bool>,
    /// The WebTest request URL.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: String,
}
