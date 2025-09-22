#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GuestPoliciesAssignmentOsType {
    /// Targets VM instances with OS Inventory enabled and having the following OS architecture.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "osArchitecture")]
    pub r#os_architecture: Option<String>,
    /// Targets VM instances with OS Inventory enabled and having the following OS short name, for example "debian" or "windows".
    #[builder(into)]
    #[serde(rename = "osShortName")]
    pub r#os_short_name: Option<String>,
    /// Targets VM instances with OS Inventory enabled and having the following following OS version.
    #[builder(into)]
    #[serde(rename = "osVersion")]
    pub r#os_version: Option<String>,
}
