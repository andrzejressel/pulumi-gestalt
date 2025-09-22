#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LocalUserPermissionScopePermissions {
    /// Specifies if the Local User has the create permission for this scope. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "create")]
    pub r#create: Option<bool>,
    /// Specifies if the Local User has the delete permission for this scope. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "delete")]
    pub r#delete: Option<bool>,
    /// Specifies if the Local User has the list permission for this scope. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "list")]
    pub r#list: Option<bool>,
    /// Specifies if the Local User has the read permission for this scope. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "read")]
    pub r#read: Option<bool>,
    /// Specifies if the Local User has the write permission for this scope. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "write")]
    pub r#write: Option<bool>,
}
