#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetRegionInstanceGroupManagerAutoHealingPolicy {
    /// The health check resource that signals autohealing.
    #[builder(into)]
    #[serde(rename = "healthCheck")]
    pub r#health_check: Box<String>,
    /// The number of seconds that the managed instance group waits before it applies autohealing policies to new instances or recently recreated instances. Between 0 and 3600.
    #[builder(into)]
    #[serde(rename = "initialDelaySec")]
    pub r#initial_delay_sec: Box<i32>,
}
