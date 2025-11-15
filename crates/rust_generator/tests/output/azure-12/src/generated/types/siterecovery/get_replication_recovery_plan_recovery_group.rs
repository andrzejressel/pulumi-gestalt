#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetReplicationRecoveryPlanRecoveryGroup {
    /// one or more `action` block. which will be executed after the group recovery.
    #[builder(into)]
    #[serde(rename = "postActions")]
    pub r#post_actions: Vec<Vec<super::super::types::siterecovery::GetReplicationRecoveryPlanRecoveryGroupPostAction>>,
    /// one or more `action` block. which will be executed before the group recovery.
    #[builder(into)]
    #[serde(rename = "preActions")]
    pub r#pre_actions: Vec<Vec<super::super::types::siterecovery::GetReplicationRecoveryPlanRecoveryGroupPreAction>>,
    /// one or more id of protected VM.
    #[builder(into)]
    #[serde(rename = "replicatedProtectedItems")]
    pub r#replicated_protected_items: Vec<String>,
    /// Type of the action detail.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
