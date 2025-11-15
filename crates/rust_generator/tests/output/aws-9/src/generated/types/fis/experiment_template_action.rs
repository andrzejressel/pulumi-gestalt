#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ExperimentTemplateAction {
    /// ID of the action. To find out what actions are supported see [AWS FIS actions reference](https://docs.aws.amazon.com/fis/latest/userguide/fis-actions-reference.html).
    #[builder(into)]
    #[serde(rename = "actionId")]
    pub r#action_id: String,
    /// Description of the action.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Friendly name of the action.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Parameter(s) for the action, if applicable. See below.
    #[builder(into)]
    #[serde(rename = "parameters")]
    pub r#parameters: Option<Vec<super::super::types::fis::ExperimentTemplateActionParameter>>,
    /// Set of action names that must complete before this action can be executed.
    #[builder(into)]
    #[serde(rename = "startAfters")]
    pub r#start_afters: Option<Vec<String>>,
    /// Action's target, if applicable. See below.
    #[builder(into)]
    #[serde(rename = "target")]
    pub r#target: Option<Box<super::super::types::fis::ExperimentTemplateActionTarget>>,
}
