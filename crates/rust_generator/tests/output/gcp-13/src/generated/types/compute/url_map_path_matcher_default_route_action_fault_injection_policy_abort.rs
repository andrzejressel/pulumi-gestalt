#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct UrlMapPathMatcherDefaultRouteActionFaultInjectionPolicyAbort {
    /// The HTTP status code used to abort the request.
    /// The value must be between 200 and 599 inclusive.
    #[builder(into)]
    #[serde(rename = "httpStatus")]
    pub r#http_status: Option<i32>,
    /// The percentage of traffic (connections/operations/requests) which will be aborted as part of fault injection.
    /// The value must be between 0.0 and 100.0 inclusive.
    #[builder(into)]
    #[serde(rename = "percentage")]
    pub r#percentage: Option<f64>,
}
