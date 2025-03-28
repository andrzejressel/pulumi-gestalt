#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetManagedHardwareSecurityModuleRoleDefinitionPermission {
    /// A list of action permission granted.
    #[builder(into)]
    #[serde(rename = "actions")]
    pub r#actions: Box<Vec<String>>,
    /// A list of data action permission granted.
    #[builder(into)]
    #[serde(rename = "dataActions")]
    pub r#data_actions: Box<Vec<String>>,
    /// A list of action permission excluded (but not denied).
    #[builder(into)]
    #[serde(rename = "notActions")]
    pub r#not_actions: Box<Vec<String>>,
    /// (Optional) A list of data action permission granted.
    #[builder(into)]
    #[serde(rename = "notDataActions")]
    pub r#not_data_actions: Box<Vec<String>>,
}
