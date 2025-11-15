#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterRestoreToPointInTime {
    /// The date and time to restore from. Value must be a time in Universal Coordinated Time (UTC) format and must be before the latest restorable time for the DB instance. Cannot be specified with `use_latest_restorable_time`.
    #[builder(into)]
    #[serde(rename = "restoreToTime")]
    pub r#restore_to_time: Option<String>,
    /// The type of restore to be performed. Valid values are `full-copy`, `copy-on-write`.
    #[builder(into)]
    #[serde(rename = "restoreType")]
    pub r#restore_type: Option<String>,
    /// The identifier of the source DB cluster from which to restore. Must match the identifier of an existing DB cluster.
    #[builder(into)]
    #[serde(rename = "sourceClusterIdentifier")]
    pub r#source_cluster_identifier: String,
    /// A boolean value that indicates whether the DB cluster is restored from the latest backup time. Defaults to `false`. Cannot be specified with `restore_to_time`.
    #[builder(into)]
    #[serde(rename = "useLatestRestorableTime")]
    pub r#use_latest_restorable_time: Option<bool>,
}
