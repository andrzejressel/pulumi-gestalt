#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ScheduledActionTargetAction {
    /// An action that runs a `PauseCluster` API operation. Documented below.
    #[builder(into)]
    #[serde(rename = "pauseCluster")]
    pub r#pause_cluster: Option<Box<super::super::types::redshift::ScheduledActionTargetActionPauseCluster>>,
    /// An action that runs a `ResizeCluster` API operation. Documented below.
    #[builder(into)]
    #[serde(rename = "resizeCluster")]
    pub r#resize_cluster: Option<Box<super::super::types::redshift::ScheduledActionTargetActionResizeCluster>>,
    /// An action that runs a `ResumeCluster` API operation. Documented below.
    #[builder(into)]
    #[serde(rename = "resumeCluster")]
    pub r#resume_cluster: Option<Box<super::super::types::redshift::ScheduledActionTargetActionResumeCluster>>,
}
