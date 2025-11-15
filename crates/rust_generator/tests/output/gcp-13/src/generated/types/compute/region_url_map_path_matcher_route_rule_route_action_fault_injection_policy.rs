#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegionUrlMapPathMatcherRouteRuleRouteActionFaultInjectionPolicy {
    /// The specification for how client requests are aborted as part of fault injection.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "abort")]
    pub r#abort: Option<Box<super::super::types::compute::RegionUrlMapPathMatcherRouteRuleRouteActionFaultInjectionPolicyAbort>>,
    /// The specification for how client requests are delayed as part of fault injection, before being sent to a backend service.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "delay")]
    pub r#delay: Option<Box<super::super::types::compute::RegionUrlMapPathMatcherRouteRuleRouteActionFaultInjectionPolicyDelay>>,
}
