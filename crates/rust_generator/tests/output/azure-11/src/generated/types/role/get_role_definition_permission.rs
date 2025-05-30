#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetRoleDefinitionPermission {
    /// A list of actions supported by this role.
    #[builder(into)]
    #[serde(rename = "actions")]
    pub r#actions: Box<Vec<String>>,
    /// The conditions on this role definition, which limits the resources it can be assigned to.
    #[builder(into)]
    #[serde(rename = "condition")]
    pub r#condition: Box<String>,
    /// The version of the condition.
    #[builder(into)]
    #[serde(rename = "conditionVersion")]
    pub r#condition_version: Box<String>,
    /// A list of data actions allowed by this role.
    #[builder(into, default)]
    #[serde(rename = "dataActions")]
    pub r#data_actions: Box<Option<Vec<String>>>,
    /// A list of actions which are denied by this role.
    #[builder(into)]
    #[serde(rename = "notActions")]
    pub r#not_actions: Box<Vec<String>>,
    /// A list of data actions which are denied by this role.
    #[builder(into, default)]
    #[serde(rename = "notDataActions")]
    pub r#not_data_actions: Box<Option<Vec<String>>>,
}
