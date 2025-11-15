#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TableMaintenanceConfigurationIcebergSnapshotManagementSettings {
    /// Snapshots older than this will be marked for deletiion.
    /// Must be at least `1`.
    #[builder(into)]
    #[serde(rename = "maxSnapshotAgeHours")]
    pub r#max_snapshot_age_hours: f64,
    /// Minimum number of snapshots to keep.
    /// Must be at least `1`.
    #[builder(into)]
    #[serde(rename = "minSnapshotsToKeep")]
    pub r#min_snapshots_to_keep: f64,
}
