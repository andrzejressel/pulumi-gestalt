#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetProfilingGroupProfilingStatus {
    #[builder(into)]
    #[serde(rename = "latestAgentOrchestratedAt")]
    pub r#latest_agent_orchestrated_at: String,
    #[builder(into)]
    #[serde(rename = "latestAgentProfileReportedAt")]
    pub r#latest_agent_profile_reported_at: String,
    #[builder(into)]
    #[serde(rename = "latestAggregatedProfiles")]
    pub r#latest_aggregated_profiles: Vec<super::super::types::codeguruprofiler::GetProfilingGroupProfilingStatusLatestAggregatedProfile>,
}
