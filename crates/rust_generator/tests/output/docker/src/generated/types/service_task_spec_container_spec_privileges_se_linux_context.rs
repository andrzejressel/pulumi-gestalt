#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceTaskSpecContainerSpecPrivilegesSeLinuxContext {
    /// Disable SELinux
    #[builder(into)]
    #[serde(rename = "disable")]
    pub r#disable: Option<bool>,
    /// SELinux level label
    #[builder(into)]
    #[serde(rename = "level")]
    pub r#level: Option<String>,
    /// SELinux role label
    #[builder(into)]
    #[serde(rename = "role")]
    pub r#role: Option<String>,
    /// SELinux type label
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
    /// SELinux user label
    #[builder(into)]
    #[serde(rename = "user")]
    pub r#user: Option<String>,
}
