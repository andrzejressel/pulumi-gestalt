#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetListenerRuleCondition {
    /// Contains a single attribute `values`, which contains a set of host names.
    #[builder(into)]
    #[serde(rename = "hostHeader")]
    pub r#host_header: Option<Box<super::super::types::lb::GetListenerRuleConditionHostHeader>>,
    /// HTTP header and values to match.
    /// Detailed below.
    #[builder(into)]
    #[serde(rename = "httpHeader")]
    pub r#http_header: Option<Box<super::super::types::lb::GetListenerRuleConditionHttpHeader>>,
    /// Contains a single attribute `values`, which contains a set of HTTP request methods.
    #[builder(into)]
    #[serde(rename = "httpRequestMethod")]
    pub r#http_request_method: Option<Box<super::super::types::lb::GetListenerRuleConditionHttpRequestMethod>>,
    /// Contains a single attribute `values`, which contains a set of path patterns to compare against the request URL.
    #[builder(into)]
    #[serde(rename = "pathPattern")]
    pub r#path_pattern: Option<Box<super::super::types::lb::GetListenerRuleConditionPathPattern>>,
    /// Query string parameters to match.
    /// Detailed below.
    #[builder(into)]
    #[serde(rename = "queryString")]
    pub r#query_string: Option<Box<super::super::types::lb::GetListenerRuleConditionQueryString>>,
    /// Contains a single attribute `values`, which contains a set of source IPs in CIDR notation.
    #[builder(into)]
    #[serde(rename = "sourceIp")]
    pub r#source_ip: Option<Box<super::super::types::lb::GetListenerRuleConditionSourceIp>>,
}
