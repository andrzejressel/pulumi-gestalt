#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkflowAccessControl {
    /// A `action` block as defined below.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Option<Box<super::super::types::logicapps::WorkflowAccessControlAction>>,
    /// A `content` block as defined below.
    #[builder(into)]
    #[serde(rename = "content")]
    pub r#content: Option<Box<super::super::types::logicapps::WorkflowAccessControlContent>>,
    /// A `trigger` block as defined below.
    #[builder(into)]
    #[serde(rename = "trigger")]
    pub r#trigger: Option<Box<super::super::types::logicapps::WorkflowAccessControlTrigger>>,
    /// A `workflow_management` block as defined below.
    #[builder(into)]
    #[serde(rename = "workflowManagement")]
    pub r#workflow_management: Option<Box<super::super::types::logicapps::WorkflowAccessControlWorkflowManagement>>,
}
