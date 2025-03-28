#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct UrlMapDefaultCustomErrorResponsePolicy {
    /// Specifies rules for returning error responses.
    /// In a given policy, if you specify rules for both a range of error codes as well as rules for specific error codes then rules with specific error codes have a higher priority.
    /// For example, assume that you configure a rule for 401 (Un-authorized) code, and another for all 4 series error codes (4XX).
    /// If the backend service returns a 401, then the rule for 401 will be applied. However if the backend service returns a 403, the rule for 4xx takes effect.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "errorResponseRules")]
    pub r#error_response_rules: Box<Option<Vec<super::super::types::compute::UrlMapDefaultCustomErrorResponsePolicyErrorResponseRule>>>,
    /// The full or partial URL to the BackendBucket resource that contains the custom error content. Examples are:
    /// https://www.googleapis.com/compute/v1/projects/project/global/backendBuckets/myBackendBucket
    /// compute/v1/projects/project/global/backendBuckets/myBackendBucket
    /// global/backendBuckets/myBackendBucket
    /// If errorService is not specified at lower levels like pathMatcher, pathRule and routeRule, an errorService specified at a higher level in the UrlMap will be used. If UrlMap.defaultCustomErrorResponsePolicy contains one or more errorResponseRules[], it must specify errorService.
    /// If load balancer cannot reach the backendBucket, a simple Not Found Error will be returned, with the original response code (or overrideResponseCode if configured).
    #[builder(into, default)]
    #[serde(rename = "errorService")]
    pub r#error_service: Box<Option<String>>,
}
