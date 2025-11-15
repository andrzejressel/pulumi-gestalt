#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct UrlMapPathMatcherRouteRuleHeaderAction {
    /// Headers to add to a matching request prior to forwarding the request to the backendService.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "requestHeadersToAdds")]
    pub r#request_headers_to_adds: Option<Vec<super::super::types::compute::UrlMapPathMatcherRouteRuleHeaderActionRequestHeadersToAdd>>,
    /// A list of header names for headers that need to be removed from the request prior to
    /// forwarding the request to the backendService.
    #[builder(into)]
    #[serde(rename = "requestHeadersToRemoves")]
    pub r#request_headers_to_removes: Option<Vec<String>>,
    /// Headers to add the response prior to sending the response back to the client.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "responseHeadersToAdds")]
    pub r#response_headers_to_adds: Option<Vec<super::super::types::compute::UrlMapPathMatcherRouteRuleHeaderActionResponseHeadersToAdd>>,
    /// A list of header names for headers that need to be removed from the response prior to sending the
    /// response back to the client.
    #[builder(into)]
    #[serde(rename = "responseHeadersToRemoves")]
    pub r#response_headers_to_removes: Option<Vec<String>>,
}
