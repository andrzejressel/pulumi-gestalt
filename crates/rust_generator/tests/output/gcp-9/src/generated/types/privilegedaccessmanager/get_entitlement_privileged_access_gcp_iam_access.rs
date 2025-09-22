#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetEntitlementPrivilegedAccessGcpIamAccess {
    /// Name of the resource.
    #[builder(into)]
    #[serde(rename = "resource")]
    pub r#resource: String,
    /// The type of this resource.
    #[builder(into)]
    #[serde(rename = "resourceType")]
    pub r#resource_type: String,
    /// Role bindings to be created on successful grant.
    #[builder(into)]
    #[serde(rename = "roleBindings")]
    pub r#role_bindings: Vec<super::super::types::privilegedaccessmanager::GetEntitlementPrivilegedAccessGcpIamAccessRoleBinding>,
}
