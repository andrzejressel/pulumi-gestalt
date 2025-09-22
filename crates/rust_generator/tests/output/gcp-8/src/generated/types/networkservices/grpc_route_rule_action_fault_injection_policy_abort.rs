#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GrpcRouteRuleActionFaultInjectionPolicyAbort {
    /// The HTTP status code used to abort the request.
    #[builder(into)]
    #[serde(rename = "httpStatus")]
    pub r#http_status: Option<i32>,
    /// The percentage of traffic which will be aborted.
    #[builder(into)]
    #[serde(rename = "percentage")]
    pub r#percentage: Option<i32>,
}
