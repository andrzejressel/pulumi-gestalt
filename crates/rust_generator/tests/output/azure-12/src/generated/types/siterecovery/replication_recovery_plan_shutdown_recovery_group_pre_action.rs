#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ReplicationRecoveryPlanShutdownRecoveryGroupPreAction {
    /// The fabric location of runbook or script. Possible values are `Primary` and `Recovery`. It must not be specified when `type` is `ManualActionDetails`.
    /// 
    /// > **NOTE:** This is required when `type` is set to `AutomationRunbookActionDetails` or `ScriptActionDetails`.
    #[builder(into)]
    #[serde(rename = "fabricLocation")]
    pub r#fabric_location: Option<String>,
    /// Directions of fail over. Possible values are `PrimaryToRecovery` and `RecoveryToPrimary`
    #[builder(into)]
    #[serde(rename = "failOverDirections")]
    pub r#fail_over_directions: Vec<String>,
    /// Types of fail over. Possible values are `TestFailover`, `PlannedFailover` and `UnplannedFailover`
    #[builder(into)]
    #[serde(rename = "failOverTypes")]
    pub r#fail_over_types: Vec<String>,
    /// Instructions of manual action.
    /// 
    /// > **NOTE:** This property is required when `type` is set to `ManualActionDetails`.
    #[builder(into)]
    #[serde(rename = "manualActionInstruction")]
    pub r#manual_action_instruction: Option<String>,
    /// The name of the Replication Plan. The name can contain only letters, numbers, and hyphens. It should start with a letter and end with a letter or a number. Can be a maximum of 63 characters. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Id of runbook.
    /// 
    /// > **NOTE:** This property is required when `type` is set to `AutomationRunbookActionDetails`.
    #[builder(into)]
    #[serde(rename = "runbookId")]
    pub r#runbook_id: Option<String>,
    /// Path of action script.
    /// 
    /// > **NOTE:** This property is required when `type` is set to `ScriptActionDetails`.
    #[builder(into)]
    #[serde(rename = "scriptPath")]
    pub r#script_path: Option<String>,
    /// Type of the action detail. Possible values are `AutomationRunbookActionDetails`, `ManualActionDetails` and `ScriptActionDetails`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
