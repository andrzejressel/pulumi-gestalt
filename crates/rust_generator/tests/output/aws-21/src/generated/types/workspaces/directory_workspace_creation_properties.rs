#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DirectoryWorkspaceCreationProperties {
    /// The identifier of your custom security group. Should relate to the same VPC, where workspaces reside in.
    #[builder(into)]
    #[serde(rename = "customSecurityGroupId")]
    pub r#custom_security_group_id: Option<String>,
    /// The default organizational unit (OU) for your WorkSpace directories. Should conform `"OU=<value>,DC=<value>,...,DC=<value>"` pattern.
    #[builder(into)]
    #[serde(rename = "defaultOu")]
    pub r#default_ou: Option<String>,
    /// Indicates whether internet access is enabled for your WorkSpaces.
    #[builder(into)]
    #[serde(rename = "enableInternetAccess")]
    pub r#enable_internet_access: Option<bool>,
    /// Indicates whether maintenance mode is enabled for your WorkSpaces. For more information, see [WorkSpace Maintenance](https://docs.aws.amazon.com/workspaces/latest/adminguide/workspace-maintenance.html)..
    #[builder(into)]
    #[serde(rename = "enableMaintenanceMode")]
    pub r#enable_maintenance_mode: Option<bool>,
    /// Indicates whether users are local administrators of their WorkSpaces.
    #[builder(into)]
    #[serde(rename = "userEnabledAsLocalAdministrator")]
    pub r#user_enabled_as_local_administrator: Option<bool>,
}
