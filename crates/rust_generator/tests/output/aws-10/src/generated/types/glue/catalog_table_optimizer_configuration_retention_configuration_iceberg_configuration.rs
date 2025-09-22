#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CatalogTableOptimizerConfigurationRetentionConfigurationIcebergConfiguration {
    /// If set to `false`, snapshots are only deleted from table metadata, and the underlying data and metadata files are not deleted. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "cleanExpiredFiles")]
    pub r#clean_expired_files: Option<bool>,
    /// The number of Iceberg snapshots to retain within the retention period. Defaults to `1` or the corresponding Iceberg table configuration field if it exists.
    #[builder(into)]
    #[serde(rename = "numberOfSnapshotsToRetain")]
    pub r#number_of_snapshots_to_retain: Option<f64>,
    /// The number of days to retain the Iceberg snapshots. Defaults to `5`, or the corresponding Iceberg table configuration field if it exists.
    #[builder(into)]
    #[serde(rename = "snapshotRetentionPeriodInDays")]
    pub r#snapshot_retention_period_in_days: Option<f64>,
}
