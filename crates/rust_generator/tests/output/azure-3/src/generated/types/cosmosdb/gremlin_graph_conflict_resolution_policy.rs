#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GremlinGraphConflictResolutionPolicy {
    /// The conflict resolution path in the case of LastWriterWins mode.
    #[builder(into)]
    #[serde(rename = "conflictResolutionPath")]
    pub r#conflict_resolution_path: Option<String>,
    /// The procedure to resolve conflicts in the case of custom mode.
    #[builder(into)]
    #[serde(rename = "conflictResolutionProcedure")]
    pub r#conflict_resolution_procedure: Option<String>,
    /// Indicates the conflict resolution mode. Possible values include: `LastWriterWins`, `Custom`.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: String,
}
