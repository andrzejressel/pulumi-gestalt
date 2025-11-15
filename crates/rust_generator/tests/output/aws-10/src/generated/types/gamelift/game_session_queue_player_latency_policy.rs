#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GameSessionQueuePlayerLatencyPolicy {
    /// Maximum latency value that is allowed for any player.
    #[builder(into)]
    #[serde(rename = "maximumIndividualPlayerLatencyMilliseconds")]
    pub r#maximum_individual_player_latency_milliseconds: i32,
    /// Length of time that the policy is enforced while placing a new game session. Absence of value for this attribute means that the policy is enforced until the queue times out.
    #[builder(into)]
    #[serde(rename = "policyDurationSeconds")]
    pub r#policy_duration_seconds: Option<i32>,
}
