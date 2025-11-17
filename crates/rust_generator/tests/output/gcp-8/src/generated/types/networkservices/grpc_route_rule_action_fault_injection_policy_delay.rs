#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GrpcRouteRuleActionFaultInjectionPolicyDelay {
    /// Specify a fixed delay before forwarding the request.
    #[builder(into)]
    #[serde(rename = "fixedDelay")]
    pub r#fixed_delay: Option<String>,
    /// The percentage of traffic on which delay will be injected.
    #[builder(into)]
    #[serde(rename = "percentage")]
    pub r#percentage: Option<i32>,
}
