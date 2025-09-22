#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GrpcRouteRuleActionFaultInjectionPolicy {
    /// Specification of how client requests are aborted as part of fault injection before being sent to a destination.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "abort")]
    pub r#abort: Option<Box<super::super::types::networkservices::GrpcRouteRuleActionFaultInjectionPolicyAbort>>,
    /// Specification of how client requests are delayed as part of fault injection before being sent to a destination.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "delay")]
    pub r#delay: Option<Box<super::super::types::networkservices::GrpcRouteRuleActionFaultInjectionPolicyDelay>>,
}
