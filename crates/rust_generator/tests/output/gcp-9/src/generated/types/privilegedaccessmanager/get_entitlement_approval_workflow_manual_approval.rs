#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetEntitlementApprovalWorkflowManualApproval {
    /// Optional. Do the approvers need to provide a justification for their actions?
    #[builder(into)]
    #[serde(rename = "requireApproverJustification")]
    pub r#require_approver_justification: bool,
    /// List of approval steps in this workflow. These steps would be followed in the specified order sequentially.  1 step is supported for now.
    #[builder(into)]
    #[serde(rename = "steps")]
    pub r#steps: Vec<super::super::types::privilegedaccessmanager::GetEntitlementApprovalWorkflowManualApprovalStep>,
}
