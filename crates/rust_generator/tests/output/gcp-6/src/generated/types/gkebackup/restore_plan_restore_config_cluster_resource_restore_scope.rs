#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RestorePlanRestoreConfigClusterResourceRestoreScope {
    /// If True, all valid cluster-scoped resources will be restored.
    /// Mutually exclusive to any other field in `clusterResourceRestoreScope`.
    #[builder(into)]
    #[serde(rename = "allGroupKinds")]
    pub r#all_group_kinds: Option<bool>,
    /// A list of cluster-scoped resource group kinds to NOT restore from the backup.
    /// If specified, all valid cluster-scoped resources will be restored except
    /// for those specified in the list.
    /// Mutually exclusive to any other field in `clusterResourceRestoreScope`.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "excludedGroupKinds")]
    pub r#excluded_group_kinds: Option<Vec<super::super::types::gkebackup::RestorePlanRestoreConfigClusterResourceRestoreScopeExcludedGroupKind>>,
    /// If True, no cluster-scoped resources will be restored.
    /// Mutually exclusive to any other field in `clusterResourceRestoreScope`.
    #[builder(into)]
    #[serde(rename = "noGroupKinds")]
    pub r#no_group_kinds: Option<bool>,
    /// A list of cluster-scoped resource group kinds to restore from the backup.
    /// If specified, only the selected resources will be restored.
    /// Mutually exclusive to any other field in the `clusterResourceRestoreScope`.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "selectedGroupKinds")]
    pub r#selected_group_kinds: Option<Vec<super::super::types::gkebackup::RestorePlanRestoreConfigClusterResourceRestoreScopeSelectedGroupKind>>,
}
