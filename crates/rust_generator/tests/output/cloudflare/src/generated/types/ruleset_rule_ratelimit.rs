#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RulesetRuleRatelimit {
    /// List of parameters that define how Cloudflare tracks the request rate for this rule.
    #[builder(into, default)]
    #[serde(rename = "characteristics")]
    pub r#characteristics: Box<Option<Vec<String>>>,
    /// Criteria for counting HTTP requests to trigger the Rate Limiting action. Uses the Firewall Rules expression language based on Wireshark display filters. Refer to the [Firewall Rules language](https://developers.cloudflare.com/firewall/cf-firewall-language) documentation for all available fields, operators, and functions.
    #[builder(into, default)]
    #[serde(rename = "countingExpression")]
    pub r#counting_expression: Box<Option<String>>,
    /// Once the request rate is reached, the Rate Limiting rule blocks further requests for the period of time defined in this field.
    #[builder(into, default)]
    #[serde(rename = "mitigationTimeout")]
    pub r#mitigation_timeout: Box<Option<i32>>,
    /// The period of time to consider (in seconds) when evaluating the request rate.
    #[builder(into, default)]
    #[serde(rename = "period")]
    pub r#period: Box<Option<i32>>,
    /// The number of requests over the period of time that will trigger the Rate Limiting rule.
    #[builder(into, default)]
    #[serde(rename = "requestsPerPeriod")]
    pub r#requests_per_period: Box<Option<i32>>,
    /// Whether to include requests to origin within the Rate Limiting count.
    #[builder(into, default)]
    #[serde(rename = "requestsToOrigin")]
    pub r#requests_to_origin: Box<Option<bool>>,
    /// The maximum aggregate score over the period of time that will trigger Rate Limiting rule.
    #[builder(into, default)]
    #[serde(rename = "scorePerPeriod")]
    pub r#score_per_period: Box<Option<i32>>,
    /// Name of HTTP header in the response, set by the origin server, with the score for the current request.
    #[builder(into, default)]
    #[serde(rename = "scoreResponseHeaderName")]
    pub r#score_response_header_name: Box<Option<String>>,
}
