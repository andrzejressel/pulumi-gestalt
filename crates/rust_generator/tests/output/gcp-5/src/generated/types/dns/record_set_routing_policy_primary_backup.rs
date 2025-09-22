#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RecordSetRoutingPolicyPrimaryBackup {
    /// The backup geo targets, which provide a regional failover policy for the otherwise global primary targets.
    /// Structure is document above.
    #[builder(into)]
    #[serde(rename = "backupGeos")]
    pub r#backup_geos: Vec<super::super::types::dns::RecordSetRoutingPolicyPrimaryBackupBackupGeo>,
    /// Specifies whether to enable fencing for backup geo queries.
    #[builder(into)]
    #[serde(rename = "enableGeoFencingForBackups")]
    pub r#enable_geo_fencing_for_backups: Option<bool>,
    /// The list of global primary targets to be health checked.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "primary")]
    pub r#primary: Box<super::super::types::dns::RecordSetRoutingPolicyPrimaryBackupPrimary>,
    /// Specifies the percentage of traffic to send to the backup targets even when the primary targets are healthy.
    #[builder(into)]
    #[serde(rename = "trickleRatio")]
    pub r#trickle_ratio: Option<f64>,
}
