#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ResolverSyncConfig {
    /// Conflict Detection strategy to use. Valid values are `NONE` and `VERSION`.
    #[builder(into)]
    #[serde(rename = "conflictDetection")]
    pub r#conflict_detection: Option<String>,
    /// Conflict Resolution strategy to perform in the event of a conflict. Valid values are `NONE`, `OPTIMISTIC_CONCURRENCY`, `AUTOMERGE`, and `LAMBDA`.
    #[builder(into)]
    #[serde(rename = "conflictHandler")]
    pub r#conflict_handler: Option<String>,
    /// Lambda Conflict Handler Config when configuring `LAMBDA` as the Conflict Handler. See Lambda Conflict Handler Config.
    #[builder(into)]
    #[serde(rename = "lambdaConflictHandlerConfig")]
    pub r#lambda_conflict_handler_config: Option<Box<super::super::types::appsync::ResolverSyncConfigLambdaConflictHandlerConfig>>,
}
