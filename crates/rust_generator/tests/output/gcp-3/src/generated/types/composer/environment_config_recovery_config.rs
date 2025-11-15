#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EnvironmentConfigRecoveryConfig {
    /// The configuration settings for scheduled snapshots.
    #[builder(into)]
    #[serde(rename = "scheduledSnapshotsConfig")]
    pub r#scheduled_snapshots_config: Option<Box<super::super::types::composer::EnvironmentConfigRecoveryConfigScheduledSnapshotsConfig>>,
}
