#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FeatureMembershipConfigmanagement {
    /// (Optional, Deprecated)
    /// Binauthz configuration for the cluster. Structure is documented below.
    /// This field will be ignored and should not be set.
    #[builder(into)]
    #[serde(rename = "binauthz")]
    pub r#binauthz: Option<Box<super::super::types::gkehub::FeatureMembershipConfigmanagementBinauthz>>,
    /// Config Sync configuration for the cluster. Structure is documented below.
    #[builder(into)]
    #[serde(rename = "configSync")]
    pub r#config_sync: Option<Box<super::super::types::gkehub::FeatureMembershipConfigmanagementConfigSync>>,
    /// Hierarchy Controller configuration for the cluster. Structure is documented below.
    /// Configuring Hierarchy Controller through the configmanagement feature is no longer recommended.
    /// Use open source Kubernetes [Hierarchical Namespace Controller (HNC)](https://github.com/kubernetes-sigs/hierarchical-namespaces) instead.
    /// Follow the [instructions](https://cloud.google.com/kubernetes-engine/enterprise/config-sync/docs/how-to/migrate-hierarchy-controller)
    /// to migrate from Hierarchy Controller to HNC.
    #[builder(into)]
    #[serde(rename = "hierarchyController")]
    pub r#hierarchy_controller: Option<Box<super::super::types::gkehub::FeatureMembershipConfigmanagementHierarchyController>>,
    /// Set this field to MANAGEMENT_AUTOMATIC to enable Config Sync auto-upgrades, and set this field to MANAGEMENT_MANUAL or MANAGEMENT_UNSPECIFIED to disable Config Sync auto-upgrades.
    #[builder(into)]
    #[serde(rename = "management")]
    pub r#management: Option<String>,
    /// Policy Controller configuration for the cluster. Structure is documented below.
    /// Configuring Policy Controller through the configmanagement feature is no longer recommended.
    /// Use the policycontroller feature instead.
    #[builder(into)]
    #[serde(rename = "policyController")]
    pub r#policy_controller: Option<Box<super::super::types::gkehub::FeatureMembershipConfigmanagementPolicyController>>,
    /// Version of Config Sync installed.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Option<String>,
}
