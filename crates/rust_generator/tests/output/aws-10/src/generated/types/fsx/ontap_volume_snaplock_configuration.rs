#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct OntapVolumeSnaplockConfiguration {
    /// Enables or disables the audit log volume for an FSx for ONTAP SnapLock volume. The default value is `false`.
    #[builder(into)]
    #[serde(rename = "auditLogVolume")]
    pub r#audit_log_volume: Option<bool>,
    /// The configuration object for setting the autocommit period of files in an FSx for ONTAP SnapLock volume. See `autocommit_period` Block for details.
    #[builder(into)]
    #[serde(rename = "autocommitPeriod")]
    pub r#autocommit_period: Option<Box<super::super::types::fsx::OntapVolumeSnaplockConfigurationAutocommitPeriod>>,
    /// Enables, disables, or permanently disables privileged delete on an FSx for ONTAP SnapLock Enterprise volume. Valid values: `DISABLED`, `ENABLED`, `PERMANENTLY_DISABLED`. The default value is `DISABLED`.
    #[builder(into)]
    #[serde(rename = "privilegedDelete")]
    pub r#privileged_delete: Option<String>,
    /// The retention period of an FSx for ONTAP SnapLock volume. See `retention_period` Block for details.
    #[builder(into)]
    #[serde(rename = "retentionPeriod")]
    pub r#retention_period: Option<Box<super::super::types::fsx::OntapVolumeSnaplockConfigurationRetentionPeriod>>,
    /// Specifies the retention mode of an FSx for ONTAP SnapLock volume. After it is set, it can't be changed. Valid values: `COMPLIANCE`, `ENTERPRISE`.
    #[builder(into)]
    #[serde(rename = "snaplockType")]
    pub r#snaplock_type: String,
    /// Enables or disables volume-append mode on an FSx for ONTAP SnapLock volume. The default value is `false`.
    #[builder(into)]
    #[serde(rename = "volumeAppendModeEnabled")]
    pub r#volume_append_mode_enabled: Option<bool>,
}
