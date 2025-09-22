#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NodePoolNodeConfigShieldedInstanceConfig {
    /// Defines whether the instance has integrity monitoring enabled.
    #[builder(into)]
    #[serde(rename = "enableIntegrityMonitoring")]
    pub r#enable_integrity_monitoring: Option<bool>,
    /// Defines whether the instance has Secure Boot enabled.
    #[builder(into)]
    #[serde(rename = "enableSecureBoot")]
    pub r#enable_secure_boot: Option<bool>,
}
