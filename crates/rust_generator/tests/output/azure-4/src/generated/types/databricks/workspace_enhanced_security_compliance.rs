#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkspaceEnhancedSecurityCompliance {
    /// Enables automatic cluster updates for this workspace. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "automaticClusterUpdateEnabled")]
    pub r#automatic_cluster_update_enabled: Option<bool>,
    /// Enables compliance security profile for this workspace. Defaults to `false`.
    /// 
    /// > **Note:** Changing the value of `compliance_security_profile_enabled` from `true` to `false` forces a replacement of the Databricks workspace.
    /// 
    /// > **Note:** The attributes `automatic_cluster_update_enabled` and `enhanced_security_monitoring_enabled` must be set to `true` in order to set `compliance_security_profile_enabled` to `true`.
    #[builder(into)]
    #[serde(rename = "complianceSecurityProfileEnabled")]
    pub r#compliance_security_profile_enabled: Option<bool>,
    /// A list of standards to enforce on this workspace. Possible values include `HIPAA` and `PCI_DSS`.
    /// 
    /// > **Note:** `compliance_security_profile_enabled` must be set to `true` in order to use `compliance_security_profile_standards`.
    /// 
    /// > **Note:** Removing a standard from the `compliance_security_profile_standards` list forces a replacement of the Databricks workspace.
    #[builder(into)]
    #[serde(rename = "complianceSecurityProfileStandards")]
    pub r#compliance_security_profile_standards: Option<Vec<String>>,
    /// Enables enhanced security monitoring for this workspace. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "enhancedSecurityMonitoringEnabled")]
    pub r#enhanced_security_monitoring_enabled: Option<bool>,
}
