#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct UrlMapDefaultRouteActionFaultInjectionPolicyDelay {
    /// Specifies the value of the fixed delay interval.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "fixedDelay")]
    pub r#fixed_delay: Box<Option<super::super::types::compute::UrlMapDefaultRouteActionFaultInjectionPolicyDelayFixedDelay>>,
    /// The percentage of traffic (connections/operations/requests) on which delay will be introduced as part of fault injection.
    /// The value must be between 0.0 and 100.0 inclusive.
    #[builder(into)]
    #[serde(rename = "percentage")]
    pub r#percentage: Option<f64>,
}
