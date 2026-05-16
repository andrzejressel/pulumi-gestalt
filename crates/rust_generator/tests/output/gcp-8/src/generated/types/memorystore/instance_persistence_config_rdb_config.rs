#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstancePersistenceConfigRdbConfig {
    /// Optional. Period between RDB snapshots.
    /// Possible values:
    /// ONE_HOUR
    /// SIX_HOURS
    /// TWELVE_HOURS
    /// TWENTY_FOUR_HOURS
    #[builder(into)]
    #[serde(rename = "rdbSnapshotPeriod")]
    pub r#rdb_snapshot_period: Option<String>,
    /// Optional. Time that the first snapshot was/will be attempted, and to which future
    /// snapshots will be aligned. If not provided, the current time will be
    /// used.
    #[builder(into)]
    #[serde(rename = "rdbSnapshotStartTime")]
    pub r#rdb_snapshot_start_time: Option<String>,
}
