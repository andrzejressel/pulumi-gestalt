#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkstationConfigHostGceInstanceShieldedInstanceConfig {
    /// Whether the instance has integrity monitoring enabled.
    #[builder(into)]
    #[serde(rename = "enableIntegrityMonitoring")]
    pub r#enable_integrity_monitoring: Option<bool>,
    /// Whether the instance has Secure Boot enabled.
    #[builder(into)]
    #[serde(rename = "enableSecureBoot")]
    pub r#enable_secure_boot: Option<bool>,
    /// Whether the instance has the vTPM enabled.
    #[builder(into)]
    #[serde(rename = "enableVtpm")]
    pub r#enable_vtpm: Option<bool>,
}
