#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ReplicationRecoveryPlanBootRecoveryGroup {
    /// one or more `action` block as defined below. which will be executed after the group recovery.
    #[builder(into)]
    #[serde(rename = "postActions")]
    pub r#post_actions: Option<Vec<super::super::types::siterecovery::ReplicationRecoveryPlanBootRecoveryGroupPostAction>>,
    /// one or more `action` block as defined below. which will be executed before the group recovery.
    #[builder(into)]
    #[serde(rename = "preActions")]
    pub r#pre_actions: Option<Vec<super::super::types::siterecovery::ReplicationRecoveryPlanBootRecoveryGroupPreAction>>,
    /// One or more protected VM IDs. It must not be specified when `type` is `Shutdown`.
    #[builder(into)]
    #[serde(rename = "replicatedProtectedItems")]
    pub r#replicated_protected_items: Option<Vec<String>>,
}
