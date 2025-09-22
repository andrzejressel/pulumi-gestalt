#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RestorePlanRestoreConfigRestoreOrder {
    /// A list of group kind dependency pairs
    /// that is used by Backup for GKE to
    /// generate a group kind restore order.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "groupKindDependencies")]
    pub r#group_kind_dependencies: Vec<super::super::types::gkebackup::RestorePlanRestoreConfigRestoreOrderGroupKindDependency>,
}
