#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstanceGceSetupShieldedInstanceConfig {
    /// Optional. Defines whether the VM instance has integrity monitoring
    /// enabled. Enables monitoring and attestation of the boot integrity of the VM
    /// instance. The attestation is performed against the integrity policy baseline.
    /// This baseline is initially derived from the implicitly trusted boot image
    /// when the VM instance is created. Enabled by default.
    #[builder(into, default)]
    #[serde(rename = "enableIntegrityMonitoring")]
    pub r#enable_integrity_monitoring: Box<Option<bool>>,
    /// Optional. Defines whether the VM instance has Secure Boot enabled.
    /// Secure Boot helps ensure that the system only runs authentic software by verifying
    /// the digital signature of all boot components, and halting the boot process
    /// if signature verification fails. Disabled by default.
    #[builder(into, default)]
    #[serde(rename = "enableSecureBoot")]
    pub r#enable_secure_boot: Box<Option<bool>>,
    /// Optional. Defines whether the VM instance has the vTPM enabled.
    /// Enabled by default.
    #[builder(into, default)]
    #[serde(rename = "enableVtpm")]
    pub r#enable_vtpm: Box<Option<bool>>,
}
