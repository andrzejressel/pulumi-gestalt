#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterClusterConfigGceClusterConfigShieldedInstanceConfig {
    /// Defines whether instances have integrity monitoring enabled.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "enableIntegrityMonitoring")]
    pub r#enable_integrity_monitoring: Option<bool>,
    /// Defines whether instances have Secure Boot enabled.
    #[builder(into)]
    #[serde(rename = "enableSecureBoot")]
    pub r#enable_secure_boot: Option<bool>,
    /// Defines whether instances have the [vTPM](https://cloud.google.com/security/shielded-cloud/shielded-vm#vtpm) enabled.
    #[builder(into)]
    #[serde(rename = "enableVtpm")]
    pub r#enable_vtpm: Option<bool>,
}
