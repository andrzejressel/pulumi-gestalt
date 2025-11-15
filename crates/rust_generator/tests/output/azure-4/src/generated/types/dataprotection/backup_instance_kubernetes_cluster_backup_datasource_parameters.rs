#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BackupInstanceKubernetesClusterBackupDatasourceParameters {
    /// Whether to include cluster scope resources during backup. Default to `false`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "clusterScopedResourcesEnabled")]
    pub r#cluster_scoped_resources_enabled: Option<bool>,
    /// Specifies the namespaces to be excluded during backup. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "excludedNamespaces")]
    pub r#excluded_namespaces: Option<Vec<String>>,
    /// Specifies the resource types to be excluded during backup. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "excludedResourceTypes")]
    pub r#excluded_resource_types: Option<Vec<String>>,
    /// Specifies the namespaces to be included during backup. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "includedNamespaces")]
    pub r#included_namespaces: Option<Vec<String>>,
    /// Specifies the resource types to be included during backup. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "includedResourceTypes")]
    pub r#included_resource_types: Option<Vec<String>>,
    /// Specifies the resources with such label selectors to be included during backup. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "labelSelectors")]
    pub r#label_selectors: Option<Vec<String>>,
    /// Whether to take volume snapshots during backup. Default to `false`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "volumeSnapshotEnabled")]
    pub r#volume_snapshot_enabled: Option<bool>,
}
