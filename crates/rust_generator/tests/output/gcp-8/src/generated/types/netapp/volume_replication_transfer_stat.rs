#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VolumeReplicationTransferStat {
    /// (Output)
    /// The elapsed time since the creation of the snapshot on the source volume that was last replicated
    /// to the destination volume. Lag time represents the difference in age of the destination volume
    /// data in relation to the source volume data.
    #[builder(into)]
    #[serde(rename = "lagDuration")]
    pub r#lag_duration: Option<String>,
    /// (Output)
    /// Size of last completed transfer in bytes.
    #[builder(into)]
    #[serde(rename = "lastTransferBytes")]
    pub r#last_transfer_bytes: Option<String>,
    /// (Output)
    /// Time taken during last completed transfer.
    #[builder(into)]
    #[serde(rename = "lastTransferDuration")]
    pub r#last_transfer_duration: Option<String>,
    /// (Output)
    /// Time when last transfer completed. A timestamp in RFC3339 UTC "Zulu" format. Examples: "2023-06-22T09:13:01.617Z".
    #[builder(into)]
    #[serde(rename = "lastTransferEndTime")]
    pub r#last_transfer_end_time: Option<String>,
    /// (Output)
    /// A message describing the cause of the last transfer failure.
    #[builder(into)]
    #[serde(rename = "lastTransferError")]
    pub r#last_transfer_error: Option<String>,
    /// (Output)
    /// Cumulative time taken across all transfers for the replication relationship.
    #[builder(into)]
    #[serde(rename = "totalTransferDuration")]
    pub r#total_transfer_duration: Option<String>,
    /// (Output)
    /// Cumulative bytes transferred so far for the replication relationship.
    #[builder(into)]
    #[serde(rename = "transferBytes")]
    pub r#transfer_bytes: Option<String>,
    /// (Output)
    /// Time when progress was updated last. A timestamp in RFC3339 UTC "Zulu" format. Examples: "2023-06-22T09:13:01.617Z".
    #[builder(into)]
    #[serde(rename = "updateTime")]
    pub r#update_time: Option<String>,
}
