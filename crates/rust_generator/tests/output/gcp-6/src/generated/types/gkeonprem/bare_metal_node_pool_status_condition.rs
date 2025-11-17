#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BareMetalNodePoolStatusCondition {
    /// (Output)
    /// Last time the condition transit from one status to another.
    #[builder(into)]
    #[serde(rename = "lastTransitionTime")]
    pub r#last_transition_time: Option<String>,
    /// Human-readable message indicating details about last transition.
    #[builder(into)]
    #[serde(rename = "message")]
    pub r#message: Option<String>,
    /// Machine-readable message indicating details about last transition.
    #[builder(into)]
    #[serde(rename = "reason")]
    pub r#reason: Option<String>,
    /// (Output)
    /// The lifecycle state of the condition.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Option<String>,
    /// Type of the condition.
    /// (e.g., ClusterRunning, NodePoolRunning or ServerSidePreflightReady)
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
}
