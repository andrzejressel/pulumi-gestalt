#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FeatureFleetDefaultMemberConfigConfigmanagementConfigSync {
    /// Enables the installation of ConfigSync. If set to true, ConfigSync resources will be created and the other ConfigSync fields will be applied if exist. If set to false, all other ConfigSync fields will be ignored, ConfigSync resources will be deleted. If omitted, ConfigSync resources will be managed depends on the presence of the git or oci field.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// Git repo configuration for the cluster
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "git")]
    pub r#git: Option<Box<super::super::types::gkehub::FeatureFleetDefaultMemberConfigConfigmanagementConfigSyncGit>>,
    /// OCI repo configuration for the cluster
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "oci")]
    pub r#oci: Option<Box<super::super::types::gkehub::FeatureFleetDefaultMemberConfigConfigmanagementConfigSyncOci>>,
    /// Set to true to enable the Config Sync admission webhook to prevent drifts. If set to `false`, disables the Config Sync admission webhook and does not prevent drifts.
    #[builder(into)]
    #[serde(rename = "preventDrift")]
    pub r#prevent_drift: Option<bool>,
    /// Specifies whether the Config Sync Repo is in hierarchical or unstructured mode
    #[builder(into)]
    #[serde(rename = "sourceFormat")]
    pub r#source_format: Option<String>,
}
