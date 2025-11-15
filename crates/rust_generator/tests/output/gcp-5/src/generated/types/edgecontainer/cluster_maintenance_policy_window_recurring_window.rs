#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterMaintenancePolicyWindowRecurringWindow {
    /// An RRULE (https://tools.ietf.org/html/rfc5545#section-3.8.5.3) for how
    /// this window recurs. They go on for the span of time between the start and
    /// end time.
    #[builder(into)]
    #[serde(rename = "recurrence")]
    pub r#recurrence: Option<String>,
    /// Represents an arbitrary window of time.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "window")]
    pub r#window: Option<Box<super::super::types::edgecontainer::ClusterMaintenancePolicyWindowRecurringWindowWindow>>,
}
