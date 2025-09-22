#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct StackInstancesStackInstanceSummary {
    /// Account ID in which the instance is deployed.
    #[builder(into)]
    #[serde(rename = "accountId")]
    pub r#account_id: Option<String>,
    /// Detailed status of the stack instance. Values include `PENDING`, `RUNNING`, `SUCCEEDED`, `FAILED`, `CANCELLED`, `INOPERABLE`, `SKIPPED_SUSPENDED_ACCOUNT`, `FAILED_IMPORT`.
    #[builder(into)]
    #[serde(rename = "detailedStatus")]
    pub r#detailed_status: Option<String>,
    /// Status of the stack instance's actual configuration compared to the expected template and parameter configuration of the stack set to which it belongs. Values include `DRIFTED`, `IN_SYNC`, `UNKNOWN`, `NOT_CHECKED`.
    #[builder(into)]
    #[serde(rename = "driftStatus")]
    pub r#drift_status: Option<String>,
    /// Organization root ID or organizational unit (OU) IDs that you specified for `deployment_targets`.
    #[builder(into)]
    #[serde(rename = "organizationalUnitId")]
    pub r#organizational_unit_id: Option<String>,
    /// Region that the stack instance is associated with.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: Option<String>,
    /// ID of the stack instance.
    #[builder(into)]
    #[serde(rename = "stackId")]
    pub r#stack_id: Option<String>,
    /// Name or unique ID of the stack set that the stack instance is associated with.
    #[builder(into)]
    #[serde(rename = "stackSetId")]
    pub r#stack_set_id: Option<String>,
    /// Status of the stack instance, in terms of its synchronization with its associated stack set. Values include `CURRENT`, `OUTDATED`, `INOPERABLE`.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Option<String>,
    /// Explanation for the specific status code assigned to this stack instance.
    #[builder(into)]
    #[serde(rename = "statusReason")]
    pub r#status_reason: Option<String>,
}
