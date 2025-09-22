#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetReplicationRecoveryPlanRecoveryGroupPreAction {
    /// The fabric location of runbook or script.
    #[builder(into)]
    #[serde(rename = "fabricLocation")]
    pub r#fabric_location: String,
    /// Directions of fail over.
    #[builder(into)]
    #[serde(rename = "failOverDirections")]
    pub r#fail_over_directions: Vec<String>,
    /// Types of fail over.
    #[builder(into)]
    #[serde(rename = "failOverTypes")]
    pub r#fail_over_types: Vec<String>,
    /// Instructions of manual action.
    #[builder(into)]
    #[serde(rename = "manualActionInstruction")]
    pub r#manual_action_instruction: String,
    /// The name of the Replication Plan.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Id of runbook.
    #[builder(into)]
    #[serde(rename = "runbookId")]
    pub r#runbook_id: String,
    /// Path of action script.
    #[builder(into)]
    #[serde(rename = "scriptPath")]
    pub r#script_path: String,
    /// Type of the action detail.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
